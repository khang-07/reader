import { FC } from "react";
import "../styles.css";

interface bottomBarData {
  handle?: Function;
  // figure out how to submit chapter upon click
}

const Bottom_Bar: FC<bottomBarData> = (props) => {
  return (
    <div className="bot-bar">
        <div className="switch"></div>
        <input className="search" placeholder="iMessage"></input>
        <div className="emoji"></div>
    </div>
  );
};

export default Bottom_Bar;