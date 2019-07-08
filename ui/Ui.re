
module NumInput = {
  [@react.component]
  let make = (~value, ~min, ~max, ~step, ~onChange) => {
    <input
      type_="number"
      min
      max={Js.Float.toString(max)}
      value={Js.Float.toString(value)}
      className=Css.(style([width(px(50))]))
      step
      onChange={evt => {
        let v = Js.Float.fromString(evt->ReactEvent.Form.target##value);
        onChange(v);
      }}
    />;
  };
};

module LogSlider = {
  [@react.component]
  let make = (~value, ~min, ~max, ~step, ~onChange) => {
    <div className=Styles.row>
      <input
        type_="range"
        min
        max={Js.Float.toString(max)}
        // value={Js.Float.toString(Js.Math.log(value))}
        value={Js.Float.toString(
          Js.Math.pow_float(~base=Js.Math._E, ~exp=value),
        )}
        step
        onChange={evt => {
          let v = Js.Float.fromString(evt->ReactEvent.Form.target##value);
          // onChange(Js.Math.pow_float(~base=Js.Math._E, ~exp=v))
          onChange(Js.Math.log(v));
        }}
      />
      {Styles.spacer(8)}
      <input
        type_="number"
        step
        className=Css.(style([fontSize(px(8)), width(px(50))]))
        // value={Js.Float.toString(Js.Math.log(value))}
        value={Js.Float.toString(value)}
        onChange={evt => {
          let v = Js.Float.fromString(evt->ReactEvent.Form.target##value);
          onChange(v);
        }}
      />
    </div>;
  };
};

module Slider = {
  [@react.component]
  let make = (~value, ~min, ~max, ~step, ~onChange) => {
    <div className=Styles.row>
      <input
        type_="range"
        min
        max={Js.Float.toString(max)}
        value={Js.Float.toString(value)}
        step
        onChange={evt => {
          let v = Js.Float.fromString(evt->ReactEvent.Form.target##value);
          onChange(v);
        }}
      />
      {Styles.spacer(8)}
      <input
        type_="number"
        step
        className=Css.(style([fontSize(px(8)), width(px(50))]))
        // value={Js.Float.toString(Js.Math.log(value))}
        value={Js.Float.toString(value)}
        onChange={evt => {
          let v = Js.Float.fromString(evt->ReactEvent.Form.target##value);
          onChange(v);
        }}
      />
    </div>;
  };
};
