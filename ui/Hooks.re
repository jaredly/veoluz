
let useState = initial => {
  React.useReducer((_, action) => action, initial);
};

let useOnChange = (value, onChange) => {
  let lastValue = React.useRef(value);
  React.useEffect2(() => {
    if (lastValue->React.Ref.current !== value) {
      lastValue->React.Ref.setCurrent(value);
      onChange(value)
    };
    None
  }, (value, lastValue->React.Ref.current));
};

let useUpdatingState = initial => {
  let (current, setState) = useState(initial);
  React.useEffect1(
    () => {
      if (initial !== current) {
        setState(initial);
      };
      None;
    },
    [|initial|],
  );
  (current, setState)
};

let useLoading = getter => {
  let (data, setData) = useState(None);
  React.useEffect1(
    () => {
      if (data != None) {
        setData(None);
      };
      getter()
      |> Js.Promise.then_(result => {
           setData(Some(result));
           Js.Promise.resolve();
         })
      |> ignore;
      None;
    },
    [|getter|],
  );
  data;
};