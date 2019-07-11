[@react.component]
let make = (~update, ~updateUi, ~config: Rust.config, ~ui: Rust.ui) => {
  <div className={Styles.join([Styles.column, Styles.control])}>
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
    {Styles.spacer(16)}
    <Walls config ui update updateUi />
    {Styles.spacer(16)}
    <LightSource config onChange=update />
  </div>;
};