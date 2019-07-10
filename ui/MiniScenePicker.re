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
      ~onExample,
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
      <Tippy content="View gallery">
        <div className=iconButton onClick={_evt => toggleGallery(true)}>
          <IonIcons.Gallery color="currentcolor" />
        </div>
      </Tippy>
      <Tippy content="Clear scene">
        <div className=iconButton onClick={_evt => onClearScene()}>
          <IonIcons.Document color="currentcolor" />
        </div>
      </Tippy>
      <Tippy content="Save scene">
        <div
          className=iconButton
          onClick={_evt => onSaveScene(Types.emptyScene)}>
          <IonIcons.Camera color="currentcolor" />
        </div>
      </Tippy>
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
      <div
        className=Css.(
          style([
            color(white),
            backgroundColor(hex("336")),
            marginLeft(px(32)),
            padding(px(Styles.Text.small)),
          ])
        )>
        {React.string("Examples")}
      </div>
      {React.array(
         Examples.ids->Belt.Array.map(id =>
           <div
             key=id
             className=Css.(
               style([
                 display(`flex),
                 flexDirection(`row),
                 padding(px(4)),
                 cursor(`pointer),
               ])
             )
             onMouseOver={_evt => hover(Examples.image(id))}
             onMouseOut={_evt => unHover()}>
             <div
               style={ReactDOMRe.Style.make(
                 ~backgroundImage="url(" ++ Examples.image(id) ++ ")",
                 (),
               )}
               onClick={_evt => {
                 let%Lets.Async.Consume res =
                   Web.fetch(Examples.config(id), Js.Obj.empty());
                 let%Lets.Async.Consume config = res->Web.json;
                 onExample(config);
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
               )
             />
           </div>
         ),
       )}
    </div>
  </div>;
};