let x = 10;

[@bs.module "localforage"]
external getItem: string => Js.Promise.t(Js.nullable('a)) = "";

[@bs.module "localforage"]
external setItem: (string, 'a) => Js.Promise.t(unit) = "";

[@bs.module "localforage"]
external keys: unit => Js.Promise.t(array(string)) = "";

type blob;
type canvas;

[@bs.send]
external toBlob: (canvas, blob => unit) => unit = "";
type element;
[@bs.val]
[@bs.scope "document"]
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
  setItem("scenes", json)
};

[@bs.val] [@bs.scope "URL"] external createObjectURL: blob => string = "";

module Scene = {
  [@react.component]
  let make = (~scene: scene, ~onSelect) => {
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
          style([display(`flex), flexDirection(`row), padding(px(4))])
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
              | Some(config) => onSelect(config)
            }
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
  let make = (~directory, ~onSelect) => {
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
         ->Belt.List.map(((key, scene)) =>
             <Scene scene onSelect key />
           )->Belt.List.toArray,
       )}
    </div>;
  };
};

module ConfigEditor = {
  [@react.component]
  let make = (~config, ~update, ~onSaveScene) => {
    let (tmpConfig, setTmpConfig) = Hooks.useUpdatingState(config);

    <div className=Css.(style([fontFamily("monospace"), whiteSpace(`pre)]))>
      <div>
        {React.string(Js.Json.stringifyAny(tmpConfig)->Opt.force)}
      </div>
      <button onClick={(_) => onSaveScene()}>
        {React.string("Save Sceen")}
      </button>
    </div>;
  };
};

let genId = () => Js.Math.random()->Js.Float.toStringWithRadix(~radix=36)->Js.String2.sliceToEnd(~from=2);
let genId = () => genId() ++ genId();

module Inner = {
  [@react.component]
  let make = (~wasm, ~directory) => {
    let (config, onChange) = Hooks.useState(wasm##initial());
    let (directory, setDirectory) = Hooks.useState(directory);

    // React.useEffect

    let onSaveScene = React.useCallback1(() => {
      let id = genId();
      let created = Js.Date.now();
      let canvas = getElementById("drawing")->asCanvas;
      canvas->toBlob(blob => {
        let fullId = created->Js.Float.toString ++ ":" ++ id;
        let%Async.Consume () = setItem(fullId ++ ":image", blob);
        let%Async.Consume () = setItem(fullId, config);
        let scenes = directory.scenes->Belt.Map.String.set(fullId, {
          id: fullId,
          modified: created,
          created,
          title: None,
          tags: Belt.Set.String.empty,
          children: [||],
          parent: None,
        });
        let directory = {...directory, scenes};
        setDirectory(directory);
        let%Async.Consume () = saveSceneInfo(directory);
      })
    }, [|config|]);

    React.useEffect0(() => {
      wasm##setup(config, onChange);
      None;
    });

    <div>
      <ConfigEditor config onSaveScene update={config => wasm##restore(config)} />
      <ScenePicker directory onSelect={config => wasm##restore(config)} />
    </div>
  };
}

module App = {
  let getKeys = () => keys();

  [@react.component]
  let make = (~wasm) => {
    let keys = Hooks.useLoading(getSceneInfo);

    switch (keys) {
    | None => <div> {React.string("Loading")} </div>
    | Some(directory) => <Inner wasm directory />
    };
  };
};

Rust.withModule(wasm =>
  // wasm##run();
  // let config = wasm##save();
  ReactDOMRe.renderToElementWithId(
    <App wasm />,
    "reason-root",
    // Js.log2("Config we got", config);
  )
);