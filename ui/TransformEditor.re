[@react.component]
let make = (~config: Rust.config, ~update, ~wasm: Rust.wasm) => {
  <div
    className={Styles.join([
      Styles.control,
      Css.(style([display(`flex), flexDirection(`column)])),
    ])}>
    <div className=Styles.title> {React.string("Scene transforms")} </div>
    <div>
      {React.string("Rotational symmetry: ")}
      <Ui.NumInput
        min=0
        value={config##transform##rotational_symmetry |> float_of_int}
        max=30.0
        onChange={v => {
          let v = int_of_float(v);
          let config = [%js.deep
            config["transform"]["rotational_symmetry"].replace(v)
          ];
          update(config, false);
        }}
      />
    </div>
    {Styles.spacer(8)}
    <div className=Styles.row>
      <input
        type_="checkbox"
        checked={config##transform##reflection}
        onChange={evt => {
          let checked = evt->ReactEvent.Form.target##checked;
          let config = [%js.deep
            config["transform"]["reflection"].replace(checked)
          ];
          update(config, false);
        }}
      />
      <div
        onClick={_ => {
          let config = [%js.deep
            config["transform"]["reflection"].replace(
              !config##transform##reflection,
            )
          ];
          update(config, false);
        }}>
        {React.string(" Reflect over y axis")}
      </div>
    </div>
    {Styles.spacer(8)}
    <div>
      {React.string("Center offset: ")}
      <Ui.NumInput
        value={config##rendering##center |> fst}
        onChange={x => {
          let config = [%js.deep
            config["rendering"]["center"].map(((_, y)) => (x, y))
          ];
          update(config, false);
        }}
      />
      <Ui.NumInput
        value={config##rendering##center |> snd}
        onChange={y => {
          let config = [%js.deep
            config["rendering"]["center"].map(((x, _)) => (x, y))
          ];
          update(config, false);
        }}
      />
    </div>
    {Styles.spacer(8)}
    <div>
      {React.string("Zoom: ")}
      <Ui.NumInput
        value={config##rendering##zoom}
        width=100
        onChange={zoom => {
          let config = [%js.deep config["rendering"]["zoom"].replace(zoom)];
          update(config, false);
        }}
      />
    </div>
    {Styles.spacer(8)}
    <ExposureFunction wasm config update />
  </div>;
};