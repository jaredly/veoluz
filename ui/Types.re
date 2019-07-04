
type scene = {
  id: string,
  modified: float,
  created: float,
  title: option(string),
  tags: Belt.Set.String.t,
  children: array(string),
  parent: option(string),
};

let emptyScene = {
  id: "",
  modified: 0.,
  created: 0.,
  title: None,
  tags: Belt.Set.String.empty,
  parent: None,
  children: [||]
}

type tag = {
  id: string,
  color: string,
  title: string,
};

type directory = {
  scenes: Belt.Map.String.t(scene),
  tags: Belt.Map.String.t(tag),
};
