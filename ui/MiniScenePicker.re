open Types;
// open ScenePicker;

[@react.component]
let make =
    (
      ~directory,
      ~current,
      ~onSelect,
      ~hover,
      ~unHover,
      ~onChangeScene,
      ~onUpdateTags,
      ~onSaveScene,
      ~onClearScene,
    ) => {
  let (gallery, toggleGallery) = Hooks.useState(false);
  let portal = Hooks.usePortal();
  <div
    className=Css.(
      style([
        flex(1),
        display(`flex),
        alignItems(`center),
        flexDirection(`row),
        // maxHeight(px(300)),
        maxWidth(px(1024)),
      ])
    )>
    <div className=Styles.column>
      <button onClick={_ => toggleGallery(true)}>
        {React.string("Gallery")}
      </button>
      {Styles.spacer(8)}
      {gallery
         ? ReactDOMRe.createPortal(
             <Gallery
               onClose={_ => toggleGallery(false)}
               onUpdateTags
               directory
               onChangeScene
             />,
             portal,
           )
         : React.null}
      <div className=Styles.row>
        <div
          className=Css.(
            style([
              width(px(50)),
              height(px(50)),
              flexShrink(0),
              display(`flex),
              alignItems(`center),
              justifyContent(`center),
              cursor(`pointer),
              color(rgba(255, 255, 255, 0.7)),
              Css.hover([color(white)]),
            ])
          )
          onClick={_evt => onClearScene()}>
          <IonIcons.Document color="currentcolor" />
        </div>
        <div
          className=Css.(
            style([
              width(px(50)),
              height(px(50)),
              flexShrink(0),
              display(`flex),
              alignItems(`center),
              justifyContent(`center),
              cursor(`pointer),
              color(rgba(255, 255, 255, 0.7)),
              Css.hover([color(white)]),
            ])
          )
          onClick={_evt => onSaveScene(Types.emptyScene)}>
          <IonIcons.Camera color="currentcolor" />
        </div>
      </div>
      {Styles.spacer(8)}
      <button className=Css.(style([visibility(`hidden)]))>
        {React.string("Hidden")}
      </button>
    </div>
    <div
      className=Css.(
        style([
          flex(1),
          display(`flex),
          alignSelf(`stretch),
          alignItems(`center),
          flexDirection(`row),
          overflow(`auto),
        ])
      )>
      {React.array(
         directory.scenes
         ->Belt.Map.String.toArray
         ->Belt.List.fromArray
         ->Belt.List.sort(((k, _), (k2, _)) => compare(k2, k))
         ->Belt.List.map(((key, scene)) =>
             <ScenePicker.Scene
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