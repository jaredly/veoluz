open Types;
// open ScenePicker;

let iconButton =
  Css.(
    style([
      width(px(50)),
      height(px(50)),
      flexShrink(0),
      display(`flex),
      alignItems(`center),
      justifyContent(`center),
      cursor(`pointer),
      color(rgba(255, 255, 255, 0.7)),
      Css.hover([
        color(white),
        backgroundColor(
          // Colors.button
          hex("335"),
        ),
      ]),
    ])
  );

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
      <div className=iconButton onClick={_evt => toggleGallery(true)}>
        <IonIcons.Gallery color="currentcolor" />
      </div>
      <div className=iconButton onClick={_evt => onClearScene()}>
        <IonIcons.Document color="currentcolor" />
      </div>
      <div
        className=iconButton onClick={_evt => onSaveScene(Types.emptyScene)}>
        <IonIcons.Camera color="currentcolor" />
      </div>
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