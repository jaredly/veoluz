let openButton = Css.(style([alignSelf(`center), display(`none)]));

module Scene = {
  [@react.component]
  let make =
      (
        ~scene: Types.scene,
        ~onChangeScene,
        ~tags,
        ~onUseScene,
        ~onUpdateTags,
        ~onClickTag,
        ~highlightedTags,
      ) => {
    let (title, updateTitle) = Hooks.useUpdatingState(scene.title);
    let url = ScenePicker.useCachedSceneImage(scene);
    <div
      style={ReactDOMRe.Style.make(
        ~backgroundImage=
          switch (url) {
          | None => ""
          | Some(url) => "url(" ++ url ++ ")"
          },
        (),
      )}
      className=Css.(
        // 500 * 576 / 1024
        style([
          hover([`selector(("." ++ openButton, [display(`block)]))]),
          margin(px(4)),
          padding(px(8)),
          width(px(500)),
          boxSizing(`borderBox),
          height(px(500)),
          backgroundColor(black),
          backgroundSize(`cover),
          position(`relative),
          display(`flex),
          flexDirection(`column),
          `declaration(("background-position", "center")),
        ])
      )>
      <div
        className={Styles.join([
          Styles.column,
          Css.(style([alignItems(`center)])),
        ])}>
        <input
          value={
            switch (title) {
            | None => ""
            | Some(t) => t
            }
          }
          className=Css.(
            style([
              backgroundColor(transparent),
              textAlign(`center),
              borderStyle(`none),
              color(white),
              padding(px(4)),
              fontSize(px(Styles.Text.small)),
            ])
          )
          placeholder="Title"
          onChange={evt => {
            let t = evt->ReactEvent.Form.target##value;
            updateTitle(t == "" ? None : Some(t));
          }}
        />
        {Styles.spacer(8)}
        <div className=Css.(style([fontSize(px(14)), color(white)]))>
          {React.string(
             Js.Date.toLocaleString(Js.Date.fromFloat(scene.created)),
           )}
        </div>
      </div>
      // <div className=Css.(style([flex(1)])) />
      {scene.title != title
         ? <div
             className={Styles.join([
               Styles.column,
               Css.(style([alignItems(`center)])),
             ])}>
             <button
               className=Css.(style([alignSelf(`center)]))
               onClick={_ => onChangeScene({...scene, title})}>
               {React.string("Save")}
             </button>
             {Styles.spacer(8)}
             <button
               className=Css.(style([alignSelf(`center)]))
               onClick={_ => updateTitle(scene.title)}>
               {React.string("Cancel")}
             </button>
           </div>
         : React.null}
      <div className=Css.(style([flex(1)])) />
      <button
        className=openButton
        onClick={_evt => {
          let%Lets.Async.Consume config = Web.LocalForage.getItem(scene.id);
          switch (Js.toOption(config)) {
          | None => ()
          | Some((config: Rust.config)) => onUseScene(scene.id, config)
          };
        }}>
        {React.string("Open")}
      </button>
      <div className=Css.(style([flex(1)])) />
      <TagsEditor
        tags
        highlightedTags
        onUpdateTags
        onClickTag
        onChange={tags => onChangeScene({...scene, tags})}
        sceneTags={scene.tags}
      />
      // the star thing
      <div
        className=Css.(
          style([
            color(scene.starred ? gold : white),
            display(`inlineBlock),
            position(`absolute),
            top(px(8)),
            left(px(8)),
            Css.hover([color(scene.starred ? white : gold)]),
            fontSize(px(30)),
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
    </div>;
  };

  let make = React.memo(make);
};

type filter = {
  star: bool,
  tags: [ | `All(Belt.Set.String.t) | `None],
};

open Belt;

let reduce = (filter, action) =>
  switch (action) {
  | `ToggleTag(id) =>
    let tags =
      switch (filter.tags) {
      | `All(t) when t->Belt.Set.String.has(id) =>
        t->Belt.Set.String.remove(id)
      | `All(t) => t->Belt.Set.String.add(id)
      | _ => Belt.Set.String.empty->Belt.Set.String.add(id)
      };
    {...filter, tags: `All(tags)};
  | `ToggleStarred => {...filter, star: !filter.star}
  | `ToggleUntagged => {
      ...filter,
      tags: filter.tags == `None ? `All(Set.String.empty) : `None,
    }
  };

let downloadZips: array((string, Rust.config, Web.blob)) => unit = [%bs.raw
  {|
(function(items) {
  var JSZip = require('jszip');
  var FileSaver = require('file-saver');
  var zip = new JSZip();

  items.forEach(function (item) {
    var folder = zip.folder(item[0]);
    folder.file("config.json", JSON.stringify(item[1]))
    folder.file("image.png", item[2]);
  })

  zip.generateAsync({type:"blob"}).then(function(content) {
    FileSaver.saveAs(content, "veoluz_scenes.zip");
  });
})
|}
];

[@react.component]
let make =
    (
      ~onClose,
      ~directory: Types.directory,
      ~onChangeScene,
      ~onUpdateTags,
      ~onUseScene,
    ) => {
  let (filter, dispatch) =
    React.useReducer(reduce, {star: false, tags: `All(Set.String.empty)});

  let onClickTag = React.useCallback(id => dispatch(`ToggleTag(id)));

  let filteredScenes =
    directory.scenes
    ->Belt.Map.String.valuesToArray
    ->Array.keep(scene =>
        (filter.star ? scene.starred : true)
        && (
          switch (filter.tags) {
          | `All(tags) =>
            tags->Set.String.every(t => scene.tags->Set.String.has(t))
          | `None => scene.tags->Set.String.isEmpty
          }
        )
      );

  <div
    className=Css.(
      style([
        position(`absolute),
        backgroundColor(black),
        top(`zero),
        bottom(`zero),
        left(`zero),
        right(`zero),
        display(`flex),
        flexDirection(`column),
      ])
    )>
    <div
      className=Css.(
        style([
          padding(px(8)),
          display(`flex),
          flexDirection(`row),
          justifyContent(`spaceBetween),
        ])
      )>
      <div
        className=Css.(
          style([
            fontSize(px(Styles.Text.large)),
            fontWeight(`normal),
            color(white),
          ])
        )>
        {React.string("Veo Luz Gallery")}
      </div>
      {Styles.spacer(8)}
      <button
        className={Styles.flatButton(Css.white)}
        onClick={_ => dispatch(`ToggleStarred)}>
        {React.string(filter.star ? "Show all" : "Starred")}
      </button>
      <button
        className={Styles.flatButtonSelectable(
          Css.white,
          filter.tags == `None,
        )}
        onClick={_ => dispatch(`ToggleUntagged)}>
        {React.string("Untagged")}
      </button>
      {Styles.spacer(8)}
      <div
        className=Css.(style([flex(1), overflow(`auto), display(`flex)]))>
        {React.array(
           directory.tags
           ->Belt.Map.String.valuesToArray
           ->Js.Array2.sortInPlaceWith((a, b) => compare(a.title, b.title))
           ->Belt.Array.map(tag => {
               let selected =
                 switch (filter.tags) {
                 | `All(t) when t->Set.String.has(tag.id) => true
                 | _ => false
                 };
               <button
                 key={tag.id}
                 className=Css.(
                   style([
                     flexShrink(0),
                     backgroundColor(
                       selected ? Colors.accent : Colors.button,
                     ),
                     marginRight(px(4)),
                   ])
                 )
                 onClick={_evt => dispatch(`ToggleTag(tag.id))}>
                 {React.string(tag.title)}
               </button>;
             }),
         )}
      </div>
      <Tippy
        content={Printf.sprintf(
          "Download %d scenes",
          Array.length(filteredScenes),
        )}>
        <div
          className=Css.(
            style([
              color(white),
              cursor(`pointer),
              hover([color(hex("aab"))]),
            ])
          )>
          <IonIcons.Compress
            className=Css.(style([paddingLeft(px(16))]))
            color="currentcolor"
            onClick={_evt => {
              let scenes =
                filteredScenes->Array.map(scene => {
                  let%Lets.Async (config, blob) =
                    Js.Promise.all2((
                      Web.LocalForage.getItem(scene.id),
                      Web.LocalForage.getItem(scene.id ++ ":image"),
                    ));
                  let title =
                    switch (scene.title) {
                    | Some(title) => title
                    | None =>
                      Js.Date.toISOString(Js.Date.fromFloat(scene.created))
                    };
                  Lets.Async.resolve((title, config, blob));
                });

              let%Lets.Async.Consume datas = scenes->Js.Promise.all;
              let datas =
                datas->Belt.Array.keepMap(((title, config, blob)) => {
                  let config = Js.Nullable.toOption(config);
                  let blob = Js.Nullable.toOption(blob);
                  switch (config, blob) {
                  | (Some(config), Some(blob)) =>
                    Some((title, config, blob))
                  | _ => None
                  };
                });
              downloadZips(datas);
              ();
            }}
          />
        </div>
      </Tippy>
      {Styles.spacer(8)}
      <div
        className=Css.(
          style([
            color(white),
            cursor(`pointer),
            hover([color(hex("aab"))]),
          ])
        )>
        <IonIcons.Close
          className=Css.(style([paddingLeft(px(0))]))
          color="currentcolor"
          onClick={_evt => onClose()}
        />
      </div>
    </div>
    <div
      className=Css.(
        style([
          flex(1),
          display(`flex),
          flexDirection(`row),
          overflowY(`auto),
          flexWrap(`wrap),
          justifyContent(`center),
        ])
      )>
      {React.array(
         filteredScenes
         ->Js.Array2.sortInPlaceWith((a, b) =>
             int_of_float(b.created -. a.created)
           )
         ->Belt.Array.map(scene =>
             <Scene
               key={scene.id}
               onChangeScene
               onClickTag
               scene
               onUpdateTags
               onUseScene
               highlightedTags={
                 switch (filter.tags) {
                 | `None => Belt.Set.String.empty
                 | `All(t) => t
                 }
               }
               tags={directory.tags}
             />
           ),
       )}
    </div>
  </div>;
};