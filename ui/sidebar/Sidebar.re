[@react.component]
let make = (~update, ~updateUi, ~config: Rust.config, ~ui) => {
  <div className=Styles.control>
    <div className=Styles.title> {React.string("Symmetry")} </div>
    <div className=Styles.row>
      <RotationSym count={config##transform##rotational_symmetry} onChange=3 />
    </div>
  </div>;
};