let x = 10;

[@bs.module "localforage"]
external getItem: string => Js.Promise.t(Js.nullable('a)) = "";

[@bs.module "localforage"]
external setItem: (string, 'a) => Js.Promise.t(unit) = "";

[@bs.module "localforage"]
external keys: unit => Js.Promise.t(array(string)) = "";

type location;
[@bs.val] external location: location = "";

[@bs.set]
external setHash: (location, string) => unit = "hash";

type blob;
type canvas;

[@bs.send] external toBlob: (canvas, blob => unit) => unit = "";
type element;
[@bs.val] [@bs.scope "document"]
external getElementById: string => element = "";
external asCanvas: element => canvas = "%identity";

module Async = {
  let let_ = (v, fn) => Js.Promise.then_(fn, v);
  let resolve = Js.Promise.resolve;
  module Consume = {
    let let_ = (v, fn) => {
      Js.Promise.then_(
        v => {
          let () = fn(v);
          Js.Promise.resolve();
        },
        v,
      )
      ->ignore;
    };
  };
};

open Types;

let sceneFromKey = key =>
  switch (Js.String.split(":", key)) {
  | [|created, id, "image"|] => {
      tags: Belt.Set.String.empty,
      id: created ++ ":" ++ id,
      title: None,
      created: float_of_string(created),
      modified: float_of_string(created),
      children: [||],
      parent: None,
    }
  | m =>
    Js.log(m);
    failwith("Bad key " ++ key);
  };

let getSceneInfo = () => {
  let%Async sceneRaw = getItem("scenes");
  switch (sceneRaw->Js.toOption) {
  | Some(sceneRaw) =>
    switch (TypeSerde.deserializeDirectory(sceneRaw)) {
    | Error(err) =>
      failwith("Invalid scene data: " ++ String.concat(" : ", err))
    | Ok(v) => Async.resolve(v)
    }
  | None =>
    let%Async keys = keys();
    Async.resolve({
      scenes:
        keys
        ->Belt.Array.keep(m => Js.String2.endsWith(m, ":image"))
        ->Belt.Array.map(key => {
            let scene = sceneFromKey(key);
            (scene.id, scene);
          })
        ->Belt.Map.String.fromArray,
      tags: Belt.Map.String.empty,
    });
  };
  // let scenesRaw =
};

let saveSceneInfo = directory => {
  let json = TypeSerde.serializeDirectory(directory);
  setItem("scenes", json);
};

[@bs.val] [@bs.scope "URL"] external createObjectURL: blob => string = "";

module Scene = {
  [@react.component]
  let make = (~scene: scene, ~selected, ~onSelect) => {
    let key = scene.id ++ ":image";
    let getter = React.useCallback1(() => getItem(key), [|key|]);
    let imageBlob = Hooks.useLoading(getter);
    let url =
      React.useMemo1(
        () =>
          switch (imageBlob) {
          | None => None
          | Some(blob) =>
            switch (Js.toOption(blob)) {
            | Some(blob) => Some(createObjectURL(blob))
            | None => Some("invalid")
            }
          },
        [|imageBlob|],
      );
    switch (url) {
    | None => <div> {React.string("Loading...")} </div>
    | Some(url) =>
      <div
        className=Css.(
          style([
            display(`flex),
            flexDirection(`row),
            padding(px(4)),
          ] @ (
            selected ? [
              backgroundColor(hex("5af"))
            ] : []
          ))
        )>
        <div
          style={ReactDOMRe.Style.make(
            ~backgroundImage="url(" ++ url ++ ")",
            (),
          )}
          onClick={_evt => {
            let%Async.Consume config = getItem(scene.id);
            switch (Js.toOption(config)) {
            | None => ()
            | Some(config) => onSelect(scene.id, config)
            };
          }}
          className=Css.(
            style([
              width(px(50)),
              backgroundColor(black),
              height(px(50)),
              backgroundSize(`cover),
              `declaration(("background-position", "center")),
              // backgroundPosition(`center, `center)
            ])
          )
        />
        {scene.children->Belt.Array.length === 0
           ? React.null
           : <div>
               {scene.children
                ->Belt.Array.map(key => <div> {React.string(key)} </div>)
                ->React.array}
             </div>}
      </div>
    };
  };
};

module Opt = {
  let force = m =>
    switch (m) {
    | None => failwith("unwrapping option")
    | Some(m) => m
    };
};

module ScenePicker = {
  [@react.component]
  let make = (~directory, ~current, ~onSelect) => {
    <div
      className=Css.(
        style([
          display(`flex),
          flexDirection(`row),
          maxHeight(px(300)),
          maxWidth(px(800)),
          overflow(`auto),
          flexWrap(`wrap),
        ])
      )>
      {React.array(
         directory.scenes
         ->Belt.Map.String.toArray
         ->Belt.List.fromArray
         ->Belt.List.sort(((k, _), (k2, _)) => compare(k2, k))
         ->Belt.List.map(((key, scene)) => <Scene selected={current == Some(key)} scene onSelect key />)
         ->Belt.List.toArray,
       )}
    </div>;
  };
};

module ConfigEditor = {
  [@react.component]
  let make = (~config, ~update, ~onSaveScene) => {
    let (tmpConfig, setTmpConfig) = Hooks.useUpdatingState(config);

    <div className=Css.(style([fontFamily("monospace"), whiteSpace(`pre)]))>
      <div> {React.string(Js.Json.stringifyAny(tmpConfig)->Opt.force)} </div>
      <button onClick={_ => onSaveScene()}>
        {React.string("Save Sceen")}
      </button>
    </div>;
  };
};

let genId = () =>
  Js.Math.random()
  ->Js.Float.toStringWithRadix(~radix=36)
  ->Js.String2.sliceToEnd(~from=2);
let genId = () => genId() ++ genId();

let newScene = () => {
  let id = genId();
  let created = Js.Date.now();
  let fullId = created->Js.Float.toString ++ ":" ++ id;
  {
    id: fullId,
    modified: created,
    created,
    title: None,
    tags: Belt.Set.String.empty,
    children: [||],
    parent: None,
  };
};

/**
Behavior:
- on first load, get the data from the hash, might be async
- on hash change that I initiate, 
- to detect hashchanges I don't initiate, should use a ref probably. Update the ref then set the hash
  */

module Inner = {
  type state = {
    directory,
    current: option(string),
    config: Rust.config,
  };


  [@react.component]
  let make = (~wasm: Rust.wasm, ~directory, ~state, ~permalink, ~updateId) => {

    // let hash = Hooks.useHash();
    // let (hashId, hashConfig) = React.useMemo1(() => {
    //   if (Js.String2.startsWith(hash, "#id=")) {
    //     let id = Js.String2.sliceToEnd(hash, ~from=4);
    //     (Some(id), getItem(id))
    //   } else if (String.length(hash) > 1) {
    //     (None, Js.Promise.resolve(wasm##parse_url_config(hash->Js.String2.sliceToEnd(~from=1))))
    //   } else {
    //     (None, Js.Promise.resolve(wasm##blank_config()))
    //   }
    // }, [|hash|]);

    let ((selectedId, config), onChange) = Hooks.useUpdatingState(state);

    let (state, dispatch) =
      React.useReducer(
        (state, action) =>
          switch (action) {
          | `Save((scene: scene)) => {
              ...state,
              directory: {
                ...state.directory,
                scenes:
                  state.directory.scenes
                  ->Belt.Map.String.set(scene.id, scene),
              },
              current: Some(scene.id),
            }
          | `Update(config) => {...state, config}
          | `Select(id) =>
            updateId(id);
            {
              ...state,
              current: Some(id)
            }
          },
        {directory, current: fst(state), config: snd(state)},
      );

    let configRef = Hooks.useOnChange(state.config, config => {
      wasm##restore(config);
    });

    Hooks.useOnChange(
      state.directory,
      directory => {
        Js.log("Directory changed -- saving");
        let%Async.Consume () = saveSceneInfo(directory);
        ();
      },
    )->ignore;

    Hooks.useOnChange(state.current, current => switch current {
      | None => ()
      | Some(id) => {
        updateId(id)
      }
    })->ignore;

    let onSaveScene =
      React.useCallback1(
        () => {
          let scene = newScene();
          let canvas = getElementById("drawing")->asCanvas;
          canvas->toBlob(blob => {
            let%Async.Consume () = setItem(scene.id ++ ":image", blob);
            let%Async.Consume () = setItem(scene.id, config);
            dispatch(`Save(scene));
            ();
          });
        },
        [|config|],
      );

    React.useEffect0(() => {
      wasm##setup(config, config => dispatch(`Update(config)));
      None;
    });

    <div>
      <ConfigEditor
        config
        onSaveScene
        update={config => wasm##restore(config)}
      />
      <ScenePicker directory=state.directory current=state.current
      onSelect={(id, config) => {
        wasm##restore(config);
        dispatch(`Select(id))
      }} />
    </div>;
  };
};

module Router = {
  // So the issue i'm having is that the hash isn't necessarily a 1:1
  // The scene ID should be a reflection of state, but the permalink shouldn't necessarily.
  // e.g. if the hash updates to a permalink, I want to imperatively update the config,
  // and probably reset the "id" to None
  // but if the hash updates to an ID, then I want to set the ID to that & load up the config.
  let loadHash = (~hash, ~wasm: Rust.wasm, ~setInitialState) => {
    if (Js.String2.startsWith(hash, "#id=")) {
      let id = Js.String2.sliceToEnd(hash, ~from=4);
      let%Async.Consume config = getItem(id);
      switch (config->Js.toOption) {
        | None => ()
        | Some(config) => setInitialState((Some(id), config))
      }
      // setInitialConfig(config);
      // setId(id);
      // (Some(id), getItem(id))
    } else if (String.length(hash) > 1) {
      let config = wasm##parse_url_config(hash->Js.String2.sliceToEnd(~from=1))->Js.toOption;
      switch config {
        | None => ()
        | Some(config) => setInitialState((None, config))
      }
      // setInitialConfig(config);
      // clearId();
      // (None, Js.Promise.resolve())
    } else {
      // ermm maybe not a reset? idk.
      setInitialState((None, wasm##blank_config()))
      // (None, Js.Promise.resolve(wasm##blank_config()))
    }
  };

  [@react.component]
  let make = (~wasm: Rust.wasm, ~blank, ~render) => {
    // let directory = Hooks.useLoading(getSceneInfo);

    let (initialState, setInitialState) = Hooks.useState((None, blank))

    let prevHash = React.useRef(None);
    let hash = Hooks.useHash();
    React.useEffect2(() => {
      if (prevHash->React.Ref.current != Some(hash)) {
        prevHash->React.Ref.setCurrent(Some(hash));
        loadHash(~hash, ~wasm, ~setInitialState)
      };
      None
    }, (hash, prevHash->React.Ref.current))

    // let (id, config) = initialState;
    render(
      ~state=initialState,
      ~permalink=hash => {
        prevHash->React.Ref.setCurrent(Some(hash));
        Web.Location.setHash(hash);
      },
      ~updateId=id => {
        let hash = "id=" ++ id;
        prevHash->React.Ref.setCurrent(Some(hash));
        Web.Location.setHash(hash);
      }
      // ~loadId=id => {
      //   Web.Location.setHash("id=" ++ id);
      // }
    )
  };
};

module App = {
  let getKeys = () => keys();

  [@react.component]
  let make = (~wasm: Rust.wasm) => {
    let keys = Hooks.useLoading(getSceneInfo);

    switch (keys) {
    | None => <div> {React.string("Loading")} </div>
    | Some(directory) =>
      <Router
        wasm
        blank={wasm##blank_config()}
        render={(~state, ~permalink, ~updateId) => {
          <Inner wasm directory state permalink updateId />
        }}
      />
    };
  };
};

Rust.withModule(wasm
  // wasm##run();
  // let config = wasm##save();
  =>
    ReactDOMRe.renderToElementWithId(
      <App wasm />,
      "reason-root",
      // Js.log2("Config we got", config);
    )
  );