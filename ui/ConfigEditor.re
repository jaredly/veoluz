
module ExposureFunction = {
  [@react.component]
  let make = (~config, ~update) => {
    <div>
      {React.string("Exposure function: ")}
      <button
        disabled={config##rendering##exposure##curve == "FourthRoot"}
        onClick={_evt => {
          let config = [%js.deep
            config["rendering"]["exposure"]["curve"].replace("FourthRoot")
          ];
          update(config, false);
        }}
        className="btn"
      >
        {React.string("Fourth Root")}
      </button>
      <button
        disabled={config##rendering##exposure##curve == "SquareRoot"}
        onClick={_evt => {
          let config = [%js.deep
            config["rendering"]["exposure"]["curve"].replace("SquareRoot")
          ];
          update(config, false);
        }}
        className="btn"
      >
        {React.string("Square Root")}
      </button>
      <button
        disabled={config##rendering##exposure##curve == "Linear"}
        onClick={_evt => {
          let config = [%js.deep
            config["rendering"]["exposure"]["curve"].replace("Linear")
          ];
          update(config, false);
        }}
        className="btn"
      >
        {React.string("Linear")}
      </button>
    </div>

  };
}

module TransformEditor = {
  [@react.component]
  let make = (~config, ~update) => {
    <div>
      {React.string("Rotational symmetry: ")}
      <input
        type_="number"
        min=0
        value={config##transform##rotational_symmetry |> string_of_int}
        max="30"
        onChange={evt => {
          let v = int_of_string((evt->ReactEvent.Form.target)##value);
          let config = [%js.deep
            config["transform"]["rotational_symmetry"].replace(v)
          ];
          update(config, false);
        }}
      />
      <br/>
      <input
        type_="checkbox"
        checked={config##transform##reflection}
        onChange={evt => {
          let checked = (evt->ReactEvent.Form.target)##checked;
          let config = [%js.deep
            config["transform"]["reflection"].replace(checked)
          ];
          update(config, false);
        }}
      />
      {React.string(" Reflect over y axis")}
    </div>
  }
};

[@react.component]
let make = (~config: Rust.config, ~wasm, ~update, ~onSaveScene) => {
  let (tmpConfig, setTmpConfig) = Hooks.useUpdatingState(config);

  <div>
    <div>
      <ExposureFunction config update />
      <TransformEditor config update />
    </div>
    // <div> {React.string(Js.Json.stringifyWithSpace(Obj.magic(tmpConfig), 2))} </div>
    <button onClick={_ => onSaveScene()}>
      {React.string("Save Sceen")}
    </button>
  </div>;
};
