[@react.component]
let make = (~config: Rust.config, ~update, ~wasm: Rust.wasm) => {
  <div
    className={Styles.join([
      Styles.control,
      Css.(style([display(`flex), flexDirection(`column)])),
    ])}>
    // <div>
    //   {React.string("Center offset: ")}
    //   <Ui.NumInput
    //     value={config##rendering##center |> fst}
    //     onChange={x => {
    //       let config = [%js.deep
    //         config["rendering"]["center"].map(((_, y)) => (x, y))
    //       ];
    //       update(config, false);
    //     }}
    //   />
    //   <Ui.NumInput
    //     value={config##rendering##center |> snd}
    //     onChange={y => {
    //       let config = [%js.deep
    //         config["rendering"]["center"].map(((x, _)) => (x, y))
    //       ];
    //       update(config, false);
    //     }}
    //   />
    // </div>
    // {Styles.spacer(8)}
    // <div>
    //   {React.string("Zoom: ")}
    //   <Ui.NumInput
    //     value={config##rendering##zoom}
    //     width=100
    //     onChange={zoom => {
    //       let config = [%js.deep config["rendering"]["zoom"].replace(zoom)];
    //       update(config, false);
    //     }}
    //   />
    // </div>
    // {Styles.spacer(8)}
     <ExposureFunction wasm config update /> </div>;
};