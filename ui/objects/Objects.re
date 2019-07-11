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
  </div>;
};