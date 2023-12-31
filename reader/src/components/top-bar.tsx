import { FC } from "react";
import "../styles.css";

interface topBarData {
    index: number;
}

const Top_Bar: FC<topBarData> = (props) => {
  return (
    <div className="top-bar">
        <div className="to-name">To: </div>
        <div className="name">{["Chapter", props.index - 1].join(" ")}</div>
        <div className="facetime-wrapper"><div className="facetime"></div></div>
        <div className="info-wrapper"><div className="info"></div></div>
    </div>
  );
};

export default Top_Bar;