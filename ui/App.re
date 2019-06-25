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
  module Consume = {
    let let_ = (v, fn) => switch v {
      | None => ()
      | Some(m) => fn(m)
    }
  }
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

let evtPos = evt => (
        evt->ReactEvent.Mouse.clientX,
        evt->ReactEvent.Mouse.clientY,
      );

let useDraggable = (~onMove) => {
  let (pressed, setPressed) = Hooks.useState(None);
  React.useEffect3(() => {
    switch pressed {
      | None => None
      | Some((x, y)) =>
        let mousemove = (evt) => onMove(evtPos(evt));
        let mouseup = (evt) => {
          // onRelease(evtPos(evt));
          setPressed(None);
        };
        Web.window->Web.addEventListener("mousemove", mousemove, true);
        Web.window->Web.addEventListener("mouseup", mouseup, true);
        Some(() => {
          Web.window->Web.removeEventListener("mousemove", mousemove, true);
          Web.window->Web.removeEventListener("mouseup", mouseup, true);
        })
    }
  }, (pressed, onMove, ()));

  (pressed, (evt) => {
    let pos = evtPos(evt);
    setPressed(Some(pos));
    // onPress(pos);
  })
};

module Draggable = {
  [@react.component]
  let make = (~render, ~onMove) => {
    let (pressed, setPressed) = Hooks.useState(None);
    let moveRef = React.useRef(onMove);
    moveRef->React.Ref.setCurrent(onMove);
    React.useEffect3(() => {
      switch pressed {
        | None => None
        | Some((x, y)) =>
          let mousemove = (evt) => React.Ref.current(moveRef)(evtPos(evt));
          let mouseup = (evt) => {
            // onRelease(evtPos(evt));
            setPressed(None);
          };
          Web.window->Web.addEventListener("mousemove", mousemove, true);
          Web.window->Web.addEventListener("mouseup", mouseup, true);
          Some(() => {
            Web.window->Web.removeEventListener("mousemove", mousemove, true);
            Web.window->Web.removeEventListener("mouseup", mouseup, true);
          })
      }
    }, (pressed, onMove, ()));

    render(~onMouseDown=(evt) => {
      let pos = evtPos(evt);
      setPressed(Some(pos));
      // onPress(pos);
    })
  }
};

let handle = () => {

};

module ConfigEditor = {
  [@react.component]
  let make = (~config: Rust.config, ~update, ~onSaveScene) => {
    let (tmpConfig, setTmpConfig) = Hooks.useUpdatingState(config);

    let containerRef = React.useRef(Js.Nullable.null);

    let (_, onMin) = useDraggable(
      ~onMove=((x, y)) => {
        let%Opt.Consume container = containerRef->React.Ref.current->Js.toOption;
        let box = Web.getBoundingClientRect(container);
        let x = float_of_int(x) -. box##left;
        let y = float_of_int(y) -. box##top;
        let xPercent = x /. (box##width);
        let config = [%js.deep config["rendering"]["exposure"]["min"].replace(xPercent)]
        update(config, false)
      }
    );

    let (_, onMax) = useDraggable(
      ~onMove=((x, y)) => {
        let%Opt.Consume container = containerRef->React.Ref.current->Js.toOption;
        let box = Web.getBoundingClientRect(container);
        let x = float_of_int(x) -. box##left;
        let y = float_of_int(y) -. box##top;
        let xPercent = x /. (box##width);
        let config = [%js.deep config["rendering"]["exposure"]["max"].replace(xPercent)]
        update(config, false)
      }
    );

    <div className=Css.(style([fontFamily("monospace"), whiteSpace(`pre)]))>
      <div
        ref=ReactDOMRe.Ref.domRef(containerRef)
      style=ReactDOMRe.Style.make(
        ~width=Js.Int.toString(config##rendering##width) ++ "px",
        ~position="relative",
        ~height="20px",
        ~backgroundColor="#afa",
        ()
      )>
        <div
          style=ReactDOMRe.Style.make(
            ~left=Js.Float.toString(float_of_int(config##rendering##width) *. config##rendering##exposure##min) ++ "px",
            ()
          )
          onMouseDown=onMin
          className=Css.(style([
            width(px(10)),
            height(px(10)),
            cursor(`ewResize),
            position(`absolute),
            backgroundColor(red)
          ]))
        />
        <div
          style=ReactDOMRe.Style.make(
            ~left=Js.Float.toString(float_of_int(config##rendering##width) *. config##rendering##exposure##max) ++ "px",
            ()
          )
          onMouseDown=onMax
          className=Css.(style([
            width(px(10)),
            height(px(10)),
            cursor(`ewResize),
            position(`absolute),
            backgroundColor(red)
          ]))
        />
      </div>
      // <div> {React.string(Js.Json.stringifyWithSpace(Obj.magic(tmpConfig), 2))} </div>
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

let hashIt: string => string = [%bs.raw {|
function(input) {
    var hash = 0;
    if (input.length == 0) {
        return hash;
    }
    for (var i = 0; i < input.length; i++) {
        var char = input.charCodeAt(i);
        hash = ((hash<<5)-hash)+char;
        hash = hash & hash; // Convert to 32bit integer
    }
    return hash;
}
|}];

let anyHash = data => {
  hashIt(Js.Json.stringifyAny(data)->Opt.force)
}

let debounced = (fn, time) => {
  let timer = ref(None);
  arg => {
    switch (timer^) {
      | None => ()
      | Some(timer) => {
        Js.Global.clearTimeout(timer)
      }
    }

    timer := Some(Js.Global.setTimeout(() => {
      fn(arg)
    }, time))
  }
};

module Router = {
  let loadHash = (~hash, ~wasm: Rust.wasm, ~onLoad) => {
    if (Js.String2.startsWith(hash, "#id=")) {
      let id = Js.String2.sliceToEnd(hash, ~from=4);
      let%Async.Consume config = getItem(id);
      switch (config->Js.toOption) {
        | None => ()
        | Some(config) => onLoad((Some(id), config))
      }
    } else if (String.length(hash) > 1) {
      let config = wasm##parse_url_config(hash->Js.String2.sliceToEnd(~from=1))->Js.toOption;
      switch config {
        | None => ()
        | Some(config) => onLoad((None, config))
      }
    } else {
      // ermm maybe not a reset? idk.
      onLoad((None, wasm##blank_config()))
    }
  };

    let updateId = (set, id) => {
      let hash = "id=" ++ id;
      set(hash);
      Web.Location.setHash(hash);
    };
    let permalink = (set, hash) => {
      set(hash);
      Web.Location.setHash(hash);
    };

  let useRouter = (~wasm: Rust.wasm, ~onLoad) => {
    let prevHash = React.useRef(None);
    let hash = Hooks.useHash();
    React.useEffect2(() => {
      if (prevHash->React.Ref.current != Some(hash)) {
        prevHash->React.Ref.setCurrent(Some(hash));
        loadHash(~hash, ~wasm, ~onLoad)
      };
      None
    }, (hash, prevHash->React.Ref.current));

    React.useCallback((newHash) => prevHash->React.Ref.setCurrent(Some(newHash)))
    // updateId = id => {
    //   let hash = "id=" ++ id;
    //   prevHash->React.Ref.setCurrent(Some(hash));
    //   Web.Location.setHash(hash);
    // }
    // ~permalink=hash => {
    //   prevHash->React.Ref.setCurrent(Some(hash));
    //   Web.Location.setHash(hash);
    // },
  };
};

module Inner = {
  type state = {
    directory,
    current: option(string),
    config: Rust.config,
  };


  [@react.component]
  let make = (~wasm: Rust.wasm, ~directory, ~blank) => {

    let router = ref((_ignored) => ());

    let (state, dispatch) =
      React.useReducer(
        (state, action) =>
          switch (action) {
          | `Route(parentId, parentConfig) =>
            {
              ...state,
              current: parentId,
              config: parentConfig
            }
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
            router^ -> Router.updateId(id);
            // updateId(id);
            {
              ...state,
              current: Some(id)
            }
          },
        {directory, current: None, config: blank},
      );

    router := Router.useRouter(~wasm, ~onLoad=((id, config)) => {
      wasm##restore(config);
      dispatch(`Route(id, config))
    });

    Hooks.useOnChange(
      state.directory,
      directory => {
        Js.log("Directory changed -- saving");
        let%Async.Consume () = saveSceneInfo(directory);
        ();
      },
    )->ignore;

    let onSaveScene =
      React.useCallback1(
        () => {
          let scene = newScene();
          let canvas = getElementById("drawing")->asCanvas;
          canvas->toBlob(blob => {
            let%Async.Consume () = setItem(scene.id ++ ":image", blob);
            let%Async.Consume () = setItem(scene.id, state.config);
            dispatch(`Save(scene));
            ();
          });
        },
        [|state.config|],
      );

    React.useEffect0(() => {
      // Js.log3("Setting up here", anyHash(state.config), state.config);
      let update = debounced(config => {
        // configRef->React.Ref.setCurrent(config);
        dispatch(`Update(config))
      }, 200);
      wasm##setup(state.config, config => {
        // Prevent a render loop
        // Js.log("Setting current from wasm (TODO debounce)");
        // configRef->React.Ref.setCurrent(config);
        // dispatch(`Update(config))
        update(config)
      });
      None;
    });

    <div>
      <ConfigEditor
        config={state.config}
        onSaveScene
        update={(config, checkpoint) => {
          // configRef->React.Ref.setCurrent(config);
          wasm##update(config, checkpoint);
          dispatch(`Update(config))
        }}
      />
      <ScenePicker directory=state.directory current=state.current
      onSelect={(id, config) => {
        Js.log3("Resetting", anyHash(config), config);
        // configRef->React.Ref.setCurrent(config);
        wasm##restore(config);
        router^ ->Router.updateId(id);
        dispatch(`Select(id))
      }} />
    </div>;
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
      <Inner wasm directory blank={wasm##blank_config()} />
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