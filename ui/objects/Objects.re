let btn = Css.(style([disabled([backgroundColor(Colors.accent)])]));

[@react.component]
let make =
    (~ui: Rust.ui, ~config: Rust.config, ~update, ~updateUi, ~wasm: Rust.wasm) => {
  <div
    className={Styles.join([
      Styles.control,
      Styles.column,
      Css.(style([flexShrink(1), overflow(`auto)])),
    ])}>
    <div className=Styles.title> {React.string("Scene objects")} </div>
    <div>
      {config##lights
       ->Belt.Array.mapWithIndex((i, light) =>
           <LightEditor
             ui
             updateUi
             wasm
             key={string_of_int(i)}
             light
             selected={
               switch (ui##selection->Js.nullToOption) {
               | None => false
               | Some(selection) =>
                 switch ([%js.deep selection["Light"]]) {
                 | None => false
                 | Some((lid, _)) => i == lid
                 }
               }
             }
             index=i
             onChange={light => {
               let config = [%js.deep
                 config["lights"].map(lights => {
                   let lights = Js.Array.copy(lights);
                   lights[i] = light;
                   lights;
                 })
               ];
               update(config, false);
             }}
           />
         )
       ->React.array}
    </div>
    <FormationEditor wasm config update />
    {Styles.spacer(8)}
    <div
      className=Css.(
        style([fontWeight(`bold), padding2(~v=px(8), ~h=`zero)])
      )>
      {React.string("Walls")}
    </div>
    <AddWall ui updateUi />
    {Styles.spacer(8)}
    <div
      className=Css.(style([flexShrink(1), overflow(`auto)]))
      onMouseEnter={_evt => wasm##show_ui()}
      onMouseLeave={_evt => wasm##hide_ui()}>
      {config##walls
       ->Belt.Array.mapWithIndex((i, wall) =>
           <WallEditor
             key={string_of_int(i)}
             ui
             updateUi
             wasm
             wall
             selected={
               switch (ui##selection->Js.nullToOption) {
               | None => false
               | Some(selection) =>
                 switch ([%js.deep selection["Wall"]]) {
                 | None => false
                 | Some((wid, _)) => i == wid
                 }
               }
             }
             index=i
             onRemove={() => {
               let config = [%js.deep
                 config["walls"].map(walls => {
                   let walls = Js.Array.copy(walls);
                   // Js.Array.removeFromInPlace(~pos=i, walls)->ignore;
                   Js.Array.removeCountInPlace(~pos=i, ~count=1, walls)
                   ->ignore;
                   walls;
                 })
               ];
               Js.log("Updating UI");
               updateUi([%js.deep ui["selection"].replace(Js.null)]);
               update(config, true);
             }}
             onChange={wall => {
               let config = [%js.deep
                 config["walls"].map(walls => {
                   let walls = Js.Array.copy(walls);
                   walls[i] = wall;
                   walls;
                 })
               ];
               update(config, false);
             }}
           />
         )
       ->React.array}
    </div>
  </div>;
};