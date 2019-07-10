[@react.component]
let make = (~enabled, ~onChange) => {
  // <div className=Styles.button>
  <button
    className={Css.style([
      Css.backgroundColor(enabled ? Colors.accent : Colors.button),
    ])}
    onClick={_ => onChange(!enabled)}>
    <svg width="50" height="50">
      <path stroke="black" strokeWidth="2px" d="M25,0 L25,50" />
      <path stroke="black" strokeWidth="2px" d="M10,50 L20,30" />
      <path
        stroke="black"
        strokeWidth="2px"
        d="M40,50 L30,30"
        strokeDasharray="2"
      />
      <path stroke="black" strokeWidth="2px" d="M0,10 L10,20" />
      <path
        stroke="black"
        strokeWidth="2px"
        d="M50,10 L40,20"
        strokeDasharray="2"
      />
    </svg>
  </button>;
};