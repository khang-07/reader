import { FC } from "react";
import "../styles.css";

const Bottom_Bar: FC = () => {
  return (
    <div className="bot-bar">
        <div className="switch"></div>
        <input className="search" placeholder="iMessage"></input>
        <div className="emoji"></div>
    </div>
  );
};

export default Bottom_Bar;