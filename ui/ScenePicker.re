open Lets;
open Types;
open Belt;

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
      starred: false,
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

let urlCache = Js.Dict.empty();

// let useSceneImage = (scene: scene) => {
//   let key = scene.id ++ ":image";
//   let getter =
//     React.useCallback2(
//       () => Web.LocalForage.getItem(key),
//       (key, scene.modified),
//     );
//   let imageBlob = Hooks.useLoading(getter);
//   let url =
//     React.useMemo1(
//       () =>
//         switch (imageBlob) {
//         | None => None
//         | Some(blob) =>
//           switch (Js.toOption(blob)) {
//           | Some(blob) => Some(createObjectURL(blob))
//           | None => Some("invalid")
//           }
//         },
//       [|imageBlob|],
//     );
//   url;
// };

let useCachedSceneImage = (scene: scene) => {
  let key = scene.id ++ ":image";
  let getter =
    React.useCallback2(
      () => Web.LocalForage.getItem(key),
      (key, scene.modified),
    );

  let (data, setData) =
    Hooks.useState(
      switch (Js.Dict.get(urlCache, key)) {
      | Some((modified, url)) when modified == scene.modified => Some(url)
      | _ => None
      },
    );

  React.useEffect3(
    () => {
      switch (Js.Dict.get(urlCache, key)) {
      | Some((modified, url)) when modified == scene.modified =>
        setData(Some(url))
      | _ =>
        if (data != None) {
          setData(None);
        };
        getter()
        |> Js.Promise.then_(result => {
             switch (Js.toOption(result)) {
             | None => ()
             | Some(blob) =>
               let url = createObjectURL(blob);
               Js.Dict.set(urlCache, key, (scene.modified, url));
               setData(Some(url));
             };
             Js.Promise.resolve();
           })
        |> ignore;
      };
      None;
    },
    (getter, key, scene.modified),
  );

  data;
};

module Scene = {
  [@react.component]
  let make =
      (
        ~scene: scene,
        ~selected,
        ~onSelect,
        ~hover,
        ~unHover,
        ~onChangeScene,
        ~onSaveScene,
      ) => {
    let url = useCachedSceneImage(scene);
    switch (url) {
    | None => <div> {React.string("Loading...")} </div>
    | Some(url) =>
      <div
        className=Css.(
          style(
            [
              display(`flex),
              flexDirection(`row),
              cursor(`pointer),
              padding(px(4)),
            ]
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
              position(`relative),
              `declaration(("background-position", "center")),
              // display(`flex),
              // justifyContent(`spaceBetween),
              // flexDirection(`column),
              // backgroundPosition(`center, `center)
            ])
          )>
          <div
            className=Css.(
              style([
                color(scene.starred ? gold : white),
                display(`inlineBlock),
                Css.hover([color(scene.starred ? white : gold)]),
                cursor(`pointer),
              ])
            )
            onClick={evt => {
              evt->ReactEvent.Mouse.stopPropagation;
              evt->ReactEvent.Mouse.preventDefault;
              onChangeScene({...scene, starred: !scene.starred});
            }}>
            {React.string(scene.starred ? {j|✭|j} : {j|☆|j})}
          </div>
          {selected
             ? <Tippy content="Update scene">
                 <div
                   className=Css.(
                     style([
                       cursor(`pointer),
                       color(rgba(255, 255, 255, 0.7)),
                       backgroundColor(black),
                       borderRadius(px(4)),
                       position(`absolute),
                       top(px(0)),
                       right(px(0)),
                       alignSelf(`center),
                       margin2(~v=px(0), ~h=`auto),
                       Css.hover([color(hex("fff"))]),
                     ])
                   )
                   onClick={evt => {
                     ReactEvent.Mouse.stopPropagation(evt);
                     onSaveScene(scene);
                   }}>
                   <IonIcons.ReverseCamera color="currentcolor" />
                 </div>
               </Tippy>
             : React.null}
        </div>
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

type filter = {
  star: bool,
  tags: Set.String.t,
};

[@react.component]
let make =
    (
      ~directory,
      ~current,
      ~onSelect,
      ~hover,
      ~unHover,
      ~onChangeScene,
      ~onSaveScene,
    ) => {
  let (filter, setFilter) =
    Hooks.useState({star: false, tags: Set.String.empty});

  <div
    className=Css.(
      style([flex(1), display(`flex), flexDirection(`column)])
    )>
    <div className={Styles.row ++ " " ++ Css.(style([padding(px(8))]))}>
      <div className=Styles.titleNoMargin>
        {React.string("Saved scenes")}
      </div>
      {Styles.spacer(8)}
      <button className={Styles.flatButton(Css.white)}>
        {React.string("Organize scenes")}
      </button>
      {Styles.spacer(8)}
      {React.string("Filters:")}
      {Styles.spacer(8)}
      <button
        className={Styles.flatButton(Css.white)}
        onClick={_ => setFilter({...filter, star: !filter.star})}>
        {React.string(filter.star ? "Show all" : "Starred")}
      </button>
    </div>
    <div
      className=Css.(
        style([
          flex(1),
          display(`flex),
          alignItems(`flexStart),
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
         ->Array.keep(((_, scene)) =>
             (filter.star ? scene.starred : true)
             && filter.tags
                ->Set.String.every(t => scene.tags->Set.String.has(t))
           )
         ->Belt.List.fromArray
         ->Belt.List.sort(((k, _), (k2, _)) => compare(k2, k))
         ->Belt.List.map(((key, scene)) =>
             <Scene
               selected={current == Some(key)}
               onSaveScene
               onChangeScene
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