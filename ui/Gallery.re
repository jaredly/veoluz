module Scene = {
  [@react.component]
  let make = (~scene: Types.scene, ~onChangeScene, ~tags, ~onUpdateTags) => {
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
      <SceneForm.TagsEditor
        tags
        onUpdateTags
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

[@react.component]
let make =
    (~onClose, ~directory: Types.directory, ~onChangeScene, ~onUpdateTags) => {
  let (filter, setFilter) =
    Hooks.useState({star: false, tags: `All(Set.String.empty)});
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
        onClick={_ => setFilter({...filter, star: !filter.star})}>
        {React.string(filter.star ? "Show all" : "Starred")}
      </button>
      <button
        className={Styles.flatButtonSelectable(
          Css.white,
          filter.tags == `None,
        )}
        onClick={_ =>
          setFilter({
            ...filter,
            tags: filter.tags == `None ? `All(Set.String.empty) : `None,
          })
        }>
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
                 onClick={_evt =>
                   if (selected) {
                     switch (filter.tags) {
                     | `All(t) =>
                       setFilter({
                         ...filter,
                         tags: `All(t->Belt.Set.String.remove(tag.id)),
                       })
                     | _ => ()
                     };
                   } else {
                     switch (filter.tags) {
                     | `All(t) =>
                       setFilter({
                         ...filter,
                         tags: `All(t->Belt.Set.String.add(tag.id)),
                       })
                     | _ =>
                       setFilter({
                         ...filter,
                         tags:
                           `All(
                             Belt.Set.String.empty->Belt.Set.String.add(
                               tag.id,
                             ),
                           ),
                       })
                     };
                   }
                 }>
                 {React.string(tag.title)}
               </button>;
             }),
         )}
      </div>
      // Styles.fullSpace
      <div
        className=Css.(
          style([
            color(white),
            cursor(`pointer),
            hover([color(hex("aab"))]),
          ])
        )>
        <IonIcons.Close color="currentcolor" onClick={_evt => onClose()} />
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
           )
         ->Js.Array2.sortInPlaceWith((a, b) =>
             int_of_float(b.created -. a.created)
           )
         ->Belt.Array.map(scene =>
             <Scene
               key={scene.id}
               onChangeScene
               scene
               onUpdateTags
               tags={directory.tags}
             />
           ),
       )}
    </div>
  </div>;
};