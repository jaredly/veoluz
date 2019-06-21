let x = 10;

[@bs.module "localforage"]
external getItem: string => Js.Promise.t(Js.nullable('a)) = "";

[@bs.module "localforage"]
external setItem: (string, 'a) => Js.Promise.t(unit) = "";

[@bs.module "localforage"]
external keys: unit => Js.Promise.t(array(string)) = "";

module Async = {
  let let_ = (v, fn) => Js.Promise.then_(fn, v);
  let resolve = Js.Promise.resolve;
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

type blob;
[@bs.val] [@bs.scope "URL"] external createObjectURL: blob => string = "";

module Scene = {
  [@react.component]
  let make = (~scene: scene) => {
    let key = scene.id ++ ":image";
    let getter =
      React.useCallback1(() => getItem(key), [|key|]);
    let imageBlob = Hooks.useLoading(getter);
    let url =
      React.useMemo1(
        () =>
          switch (imageBlob) {
          | None => None
          | Some(blob) => switch (Js.toOption(blob)) {
            | Some(blob) => Some(createObjectURL(blob))
            | None => Some("invalid")
          }
          },
        [|imageBlob|],
      );
    switch (url) {
    | None => <div> {React.string("Loading...")} </div>
    | Some(url) =>
      <div className=Css.(style([
        display(`flex),
        flexDirection(`row),
        padding(px(4))
      ]))>
        <div 
          style=ReactDOMRe.Style.make(
            ~backgroundImage="url(" ++ url ++ ")",
            ()
          )
          className=Css.(style([
            width(px(50)),
            backgroundColor(black),
            height(px(50)),
            backgroundSize(`cover),
            `declaration(("background-position", "center"))
            // backgroundPosition(`center, `center)
          ]))
        />
        {scene.children->Belt.Array.length === 0 ? React.null : <div>
          {scene.children->Belt.Array.map(key => <div>{React.string(key)}</div>)->React.array}
        </div>}
      </div>
    };
  };
};

module Opt =  {
  let force = m => switch m {
    | None => failwith("unwrapping option")
    | Some(m) => m
  }
}

module ScenePicker = {

  [@react.component]
  let make = (~scenes, ~tags) => {
    <div>
      {React.array(
          scenes
          ->Belt.Map.String.toArray
          ->Belt.Array.map(((_key, scene)) => <div>
          <Scene scene />
          </div>),
        )}
    </div>
  }
};

module ConfigEditor = {
  [@react.component]
  let make = (~config, ~update) => {
    let (tmpConfig, setTmpConfig) = Hooks.useState(config);
    React.useEffect1(() => {
      if (config != tmpConfig) {
        setTmpConfig(config)
      };
      None
    }, [|config|]);

    <div className=Css.(style([fontFamily("monospace"), whiteSpace(`pre)]))>
      {React.string(Js.Json.stringifyAny(tmpConfig)->Opt.force)}
    </div>
  };
};

module App = {
  let getKeys = () => keys();

  [@react.component]
  let make = (~wasm) => {
    let keys = Hooks.useLoading(getSceneInfo);
    let (config, onChange) = Hooks.useState(wasm##initial());
    Js.log("Rendering app here");

    React.useEffect0(() => {
      wasm##setup(config, onChange);
      // wasm.run(onChange);
      None
    });

    switch (keys) {
    | None => <div> {React.string("Loading")} </div>
    | Some({scenes, tags}) =>
      <div>
        <ConfigEditor config update={config => wasm##restore(config)} />
        <ScenePicker scenes tags />
      </div>
    };
  };
};

Rust.withModule(wasm => {
  // wasm##run();
  // let config = wasm##save();
  ReactDOMRe.renderToElementWithId(<App wasm />, "reason-root");
  // Js.log2("Config we got", config);
});