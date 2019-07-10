//

let size = 20;

let mirror =
  <svg width={string_of_int(size)} height={string_of_int(size)}>
    <rect
      x="13"
      y="0"
      width="3"
      height={string_of_int(size)}
      fill="black"
      stroke="black"
      strokeWidth="0px"
    />
    <path stroke="black" strokeWidth="1px" d="M3,3 L13,10 L3,17" fill="none" />
  </svg>;

let prism =
  <svg width={string_of_int(size)} height={string_of_int(size)}>
    <rect
      x="10"
      y="0"
      width="3"
      height={string_of_int(size)}
      fill="black"
      stroke="black"
      strokeWidth="0px"
    />
    <path
      stroke="black"
      strokeWidth="1px"
      d="M0,3 L10,10 L20,12"
      fill="none"
    />
  </svg>;

let block =
  <svg width={string_of_int(size)} height={string_of_int(size)}>
    <rect
      x="13"
      y="0"
      width="3"
      height={string_of_int(size)}
      fill="black"
      stroke="black"
      strokeWidth="0px"
    />
    <path stroke="black" strokeWidth="1px" d="M3,3 L13,10" fill="none" />
  </svg>;

let getStatus = properties => {
  switch (properties##absorb, properties##reflect) {
  | (1.0, _) => `Absorb
  | (0.0, 1.0) => `Reflect
  | (0.0, 0.0) => `Refract
  | _ => `Custom
  };
};

[@react.component]
let make = (~wall, ~onChange) => {
  let status = getStatus(wall##properties);
  <div className=Styles.column>
    <div
      className={Styles.join([
        Styles.row,
        Css.(style([padding2(~v=px(0), ~h=px(8)), marginBottom(px(8))])),
      ])}>
      <div className=Css.(style([flex(1)]))>
        {React.string(WallEditor.wallType(wall##kind))}
      </div>
      {Styles.spacer(8)}
      <button className={Styles.iconButton(status == `Reflect)}>
        mirror
      </button>
      {Styles.spacer(4)}
      <button className={Styles.iconButton(status == `Refract)}>
        prism
      </button>
      {Styles.spacer(4)}
      <button className={Styles.iconButton(status == `Absorb)}>
        block
      </button>
      {Styles.spacer(4)}
      <button className={Styles.iconButton(status == `Custom)}>
        <IonIcons.Settings fontSize="20px" />
      </button>
      Styles.fullSpace
      {wall##hide
         ? <IonIcons.EyeOff
             fontSize="20px"
             onClick={_ =>
               onChange([%js.deep wall["hide"].replace(false)], true)
             }
           />
         : <IonIcons.Eye
             fontSize="20px"
             onClick={_ =>
               onChange([%js.deep wall["hide"].replace(true)], true)
             }
           />}
      <IonIcons.Close fontSize="20px" />
    </div>
    {switch (status) {
     | `Absorb => React.null
     | `Reflect =>
       <WallEditor.ScatterEditor
         wall
         onChange={wall => onChange(wall, false)}
       />
     | `Refract =>
       <WallEditor.RefractionEditor
         wall
         onChange={wall => onChange(wall, false)}
       />
     | `Custom =>
       <WallEditor.PropertiesTriangle
         wall
         onChange={wall => onChange(wall, false)}
       />
     }}
  </div>;
};