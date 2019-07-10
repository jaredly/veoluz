[@react.component]
let make = (~update, ~updateUi, ~config: Rust.config, ~ui) => {
  <div className=Styles.control>
    <div className=Styles.title> {React.string("Symmetry")} </div>
    <div className=Styles.row>
      <RotationSym
        count={config##transform##rotational_symmetry}
        onChange={v =>
          update(
            [%js.deep config["transform"]["rotational_symmetry"].replace(v)],
            true,
          )
        }
      />
      {Styles.spacer(16)}
      <Reflection
        enabled={config##transform##reflection}
        onChange={v =>
          update(
            [%js.deep config["transform"]["reflection"].replace(v)],
            true,
          )
        }
      />
    </div>
  </div>;
};