//

let size = 20;

let mirror =
  <svg width={string_of_int(size)} height={string_of_int(size)}>
    <rect
      x="13"
      y="0"
      width="3"
      height={string_of_int(size)}
      fill="currentcolor"
      stroke="currentcolor"
      strokeWidth="0px"
    />
    <path
      stroke="currentcolor"
      strokeWidth="1px"
      d="M3,3 L13,10 L3,17"
      fill="none"
    />
  </svg>;

let prism =
  <svg width={string_of_int(size)} height={string_of_int(size)}>
    <rect
      x="10"
      y="0"
      width="3"
      height={string_of_int(size)}
      fill="currentcolor"
      stroke="currentcolor"
      strokeWidth="0px"
    />
    <path
      stroke="currentcolor"
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
      fill="currentcolor"
      stroke="currentcolor"
      strokeWidth="0px"
    />
    <path
      stroke="currentcolor"
      strokeWidth="1px"
      d="M3,3 L13,10"
      fill="none"
    />
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
let make =
    (
      ~wall,
      ~onChange,
      ~onRemove,
      ~selected,
      ~onHoverOut,
      ~onSelect,
      ~onDeselect,
      ~onHover,
    ) => {
  let status = getStatus(wall##properties);
  <div
    onMouseOver={_evt => onHover()}
    onMouseOut={_evt => onHoverOut()}
    className={Styles.join([
      Styles.column,
      Css.(style([marginBottom(px(8))])),
    ])}>
    <div
      className={Styles.join([
        Styles.row,
        Css.(style([alignItems(`center)])),
      ])}>
      <div
        className=Css.(
          style([
            cursor(`pointer),
            flex(1),
            color(selected ? Colors.accent : Colors.text),
            display(`flex),
            alignItems(`center),
            flexDirection(`row),
          ])
        )
        onClick={_evt =>
          if (selected) {
            onDeselect();
          } else {
            onSelect();
          }
        }>
        {selected
           ? <IonIcons.ArrowDown fontSize="14px" />
           : <IonIcons.ArrowRight fontSize="14px" />}
        {Styles.spacer(4)}
        {React.string(WallEditor.wallType(wall##kind))}
      </div>
      {Styles.spacer(8)}
      <Tippy content="Mirror">
        <span>
          <button
            className=Styles.colorButton
            disabled={status == `Reflect}
            onClick={_evt =>
              onChange(
                [%js.deep
                  wall["properties"].map(p =>
                    p["absorb"].replace(0.0)["reflect"].replace(1.0)
                  )
                ],
                true,
              )
            }>
            mirror
          </button>
        </span>
      </Tippy>
      // {Styles.spacer(4)}
      <Tippy content="Prism">
        <span>
          <button
            className=Styles.colorButton
            disabled={status == `Refract}
            onClick={_evt =>
              onChange(
                [%js.deep
                  wall["properties"].map(p =>
                    p["absorb"].replace(0.0)["reflect"].replace(0.0)
                  )
                ],
                true,
              )
            }>
            prism
          </button>
        </span>
      </Tippy>
      // {Styles.spacer(4)}
      <Tippy content="Block">
        <span>
          <button
            className=Styles.colorButton
            disabled={status == `Absorb}
            onClick={_evt =>
              onChange(
                [%js.deep
                  wall["properties"].map(p => p["absorb"].replace(1.0))
                ],
                true,
              )
            }>
            block
          </button>
        </span>
      </Tippy>
      // {Styles.spacer(4)}
      <Tippy content="Custom behavior">
        <span>
          <button
            className=Styles.colorButton
            disabled={status == `Custom}
            onClick={_evt =>
              onChange(
                [%js.deep
                  wall["properties"].map(p =>
                    p["absorb"].replace(0.333)["reflect"].replace(0.5)
                  )
                ],
                true,
              )
            }>
            <IonIcons.Settings fontSize="20px" color="currentcolor" />
          </button>
        </span>
      </Tippy>
      Styles.fullSpace
      {wall##hide
         ? <Tippy content="Show">
             <button
               onClick={_ =>
                 onChange([%js.deep wall["hide"].replace(false)], true)
               }
               className=Styles.colorButton>
               <IonIcons.EyeOff fontSize="20px" />
             </button>
           </Tippy>
         : <Tippy content="Hide">
             <button
               className=Styles.colorButton
               onClick={_ =>
                 onChange([%js.deep wall["hide"].replace(true)], true)
               }>
               <IonIcons.Eye fontSize="20px" />
             </button>
           </Tippy>}
      {Styles.spacer(8)}
      <Tippy content="Delete">
        <button onClick={_ => onRemove()} className=Styles.colorButton>
          <IonIcons.Close fontSize="20px" />
        </button>
      </Tippy>
    </div>
    {
      let blurb = Css.(style([fontSize(`percent(60.0))]));
      let s =
        Css.(
          style([
            padding(px(16)),
            border(px(2), `solid, Colors.accent),
            margin(px(8)),
            display(`flex),
            flexDirection(`column),
          ])
        );
      selected
        ? switch (status) {
          | `Absorb =>
            <div className=s>
              <div className=blurb>
                {React.string("This wall absorbes all light on contact.")}
              </div>
            </div>
          | `Reflect =>
            <div className=s>
              <div className=blurb>
                {React.string(
                   "This wall acts as a mirror, with all light bouncing off of it. The scatter percentage determines how much light is reflected directly (no scatter) and how much bounces off at a random angle.",
                 )}
              </div>
              {Styles.spacer(16)}
              <WallEditor.ScatterEditor
                wall
                wide=true
                onChange={wall => onChange(wall, false)}
              />
            </div>
          | `Refract =>
            <div className=s>
              <div className=blurb>
                {React.string(
                   "This wall acts as a prism. Light will pass through, and refract or reflect according to the index of refraction and the light ray's angle of incidence. ",
                 )}
              </div>
              {Styles.spacer(16)}
              <WallEditor.RefractionEditor
                wall
                onChange={wall => onChange(wall, false)}
              />
            </div>
          | `Custom =>
            <div className=s>
              <WallEditor.PropertiesTriangle
                wall
                onChange={wall => onChange(wall, false)}
              />
            </div>
          }
        : React.null;
    }
  </div>;
};