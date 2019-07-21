module NumInput = {
  [@react.component]
  let make = (~width=50, ~value, ~min=?, ~max=?, ~step=?, ~onChange) => {
    let (tmp, setTmp) =
      Hooks.useUpdatingState(
        Js.Float.toFixedWithPrecision(value, ~digits=3),
      );
    <input
      type_="number"
      ?min
      max=?{
        switch (max) {
        | None => None
        | Some(max) => Some(Js.Float.toFixedWithPrecision(max, ~digits=3))
        }
      }
      value=tmp
      className={Css.style([Css.width(Css.px(width))])}
      ?step
      onChange={evt => {
        let text = evt->ReactEvent.Form.target##value;
        let num = Js.Float.fromString(text);
        if (Js.Float.isNaN(num)
            || text == ""
            || num == value
            || abs_float(
                 num -. Js.Float.fromString(Js.Float.toString(num)),
               )
            > 0.1) {
          setTmp(text);
        } else {
          onChange(num);
        };
      }}
      // if (text == "" || Js.Float.fromString(text)->Js.Float.isNaN)
      // try (
      //   {
      //     let v = Js.Float.fromString(text);
      //   }
      // ) {
      // | _ => setTmp(text)
      // };
    />;
  };
};

module LogSlider = {
  [@react.component]
  let make = (~value, ~disabled=?, ~min, ~max, ~step, ~onChange) => {
    <div className=Styles.row>
      <input
        type_="range"
        min
        ?disabled
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
        ?disabled
        className=Css.(style([fontSize(px(8)), width(px(50))]))
        // value={Js.Float.toString(Js.Math.log(value))}
        value={Js.Float.toFixedWithPrecision(value, ~digits=3)}
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
  let make =
      (
        ~vertical=false,
        ~disabled=?,
        ~width=?,
        ~value,
        ~min,
        ~max,
        ~step,
        ~onChange,
      ) => {
    <div className={vertical ? Styles.column : Styles.row}>
      <input
        type_="range"
        min
        ?disabled
        max={Js.Float.toString(max)}
        value={Js.Float.toString(value)}
        style=?{
          switch (width) {
          | None => None
          | Some(num) =>
            Some(
              ReactDOMRe.Style.make(~width=string_of_int(num) ++ "px", ()),
            )
          }
        }
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
        ?disabled
        className=Css.(style([fontSize(px(8)), width(px(50))]))
        // value={Js.Float.toString(Js.Math.log(value))}
        value={Js.Float.toFixedWithPrecision(value, ~digits=3)}
        onChange={evt => {
          let v = Js.Float.fromString(evt->ReactEvent.Form.target##value);
          onChange(v);
        }}
      />
    </div>;
  };
};