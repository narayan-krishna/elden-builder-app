// statlist component
// users should be able to change the props entered here. BUT they should not go beyond starter class
function StatList(props) {
  return (
    props.statListData && <div>
      <p> level: {props.statListData.level}</p>
      <p> vigor: {props.statListData.vigor}</p>
      <p> mind: {props.statListData.mind}</p>
      <p> endurance: {props.statListData.endurance}</p>
      <p> strength: {props.statListData.strength}</p>
      <p> dexterity: {props.statListData.dexterity}</p>
      <p> intelligence: {props.statListData.intelligence}</p>
      <p> faith: {props.statListData.faith}</p>
      <p> arcane: {props.statListData.arcane}</p>
      <p> class: {props.statListData.class}</p>
    </div>
  );
}

function OptimizedStatlist(props) {
  return (
    props.statListData && <div>
      <p> level: {props.statListData.level}</p>
      <p> vigor: {props.statListData.vigor}</p>
      <p> mind: {props.statListData.mind}</p>
      <p> endurance: {props.statListData.endurance}</p>
      <p> strength: {props.statListData.strength}</p>
      <p> dexterity: {props.statListData.dexterity}</p>
      <p> intelligence: {props.statListData.intelligence}</p>
      <p> faith: {props.statListData.faith}</p>
      <p> arcane: {props.statListData.arcane}</p>
      <p> class: {props.statListData.class}</p>
    </div>
  );
}

export {StatList as default, OptimizedStatlist}
