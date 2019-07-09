module Scene = {
  [@react.component]
  let make = (~scene, ~onChangeScene) => {
    let (tmpScene, updateScene) = Hooks.useUpdatingState(scene);
    let url = ScenePicker.useSceneImage(scene);
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
            switch (tmpScene.title) {
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
              fontSize(px(16)),
            ])
          )
          placeholder="Title"
          onChange={evt => {
            let t = evt->ReactEvent.Form.target##value;
            updateScene({...tmpScene, title: t == "" ? None : Some(t)});
          }}
        />
        {Styles.spacer(8)}
        <div className=Css.(style([fontSize(px(14)), color(white)]))>
          {React.string(
             Js.Date.toLocaleString(Js.Date.fromFloat(scene.created)),
           )}
        </div>
      </div>
      <div className=Css.(style([flex(1)])) />
      {scene != tmpScene
         ? <button
             className=Css.(style([alignSelf(`center)]))
             onClick={_ => onChangeScene(tmpScene)}>
             {React.string("Save")}
           </button>
         : React.null}
      <div className=Css.(style([flex(1)])) />
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
};

[@react.component]
let make = (~onClose, ~directory: Types.directory, ~onChangeScene) => {
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
          style([fontSize(px(16)), fontWeight(`normal), color(white)])
        )>
        {React.string("Veo Luz Gallery")}
      </div>
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
        ])
      )>
      {React.array(
         directory.scenes
         ->Belt.Map.String.valuesToArray
         ->Js.Array2.sortInPlaceWith((a, b) =>
             int_of_float(b.created -. a.created)
           )
         ->Belt.Array.map(scene => <Scene onChangeScene scene />),
       )}
    </div>
  </div>;
};