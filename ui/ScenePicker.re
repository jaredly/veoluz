open Lets;
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
  let%Async sceneRaw = Web.LocalForage.getItem("scenes");
  switch (sceneRaw->Js.toOption) {
  | Some(sceneRaw) =>
    switch (TypeSerde.deserializeDirectory(sceneRaw)) {
    | Error(err) =>
      failwith("Invalid scene data: " ++ String.concat(" : ", err))
    | Ok(v) => Async.resolve(v)
    }
  | None =>
    let%Async keys = Web.LocalForage.keys();
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
  Web.LocalForage.setItem("scenes", json);
};

[@bs.val] [@bs.scope "URL"] external createObjectURL: Web.blob => string = "";

module Scene = {
  [@react.component]
  let make = (~scene: scene, ~selected, ~onSelect, ~hover, ~unHover) => {
    let key = scene.id ++ ":image";
    let getter =
      React.useCallback2(
        () => Web.LocalForage.getItem(key),
        (key, scene.modified),
      );
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
          style(
            [display(`flex), flexDirection(`row), padding(px(4))]
            @ (selected ? [backgroundColor(hex("5af"))] : []),
          )
        )
        onMouseOver={_evt => hover(url)}
        onMouseOut={_evt => unHover()}>
        <div
          style={ReactDOMRe.Style.make(
            ~backgroundImage="url(" ++ url ++ ")",
            (),
          )}
          onClick={_evt => {
            let%Async.Consume config = Web.LocalForage.getItem(scene.id);
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

[@react.component]
let make = (~directory, ~current, ~onSelect, ~hover, ~unHover) => {
  <div
    className=Css.(
      style([flex(1), display(`flex), flexDirection(`column)])
    )>
    <div className=(Styles.row ++ " " ++ Css.(style([
      padding(px(8))
    ])))>
      <div className=Styles.titleNoMargin>
        {React.string("Saved scenes")}
      </div>
      {Styles.spacer(8)}
      <button className=Styles.flatButton(Css.white)>
        {React.string("Organize scenes")}
      </button>
    </div>
    <div
      className=Css.(
        style([
          flex(1),
          display(`flex),
          flexDirection(`row),
          maxHeight(px(300)),
          maxWidth(px(1024)),
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
             <Scene
               selected={current == Some(key)}
               scene
               onSelect
               key
               hover
               unHover
             />
           )
         ->Belt.List.toArray,
       )}
    </div>
  </div>;
};