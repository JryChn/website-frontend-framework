import { Component } from "react";
import "./Gallary.css";

export default class Gallary extends Component {
  componentDidMount() {
    document.getElementById("gallary").style.display = "flex";
    document.getElementById("gallary").style.flexFlow = "column nowrap";
    document.getElementById("gallary-words").style.height = "100px";
    document.getElementById("gallary-words").style.display = "flex";
    document.getElementById("gallary-words").style.justifyContent = "center";
    document.getElementById("gallary-words").style.alignItems = "center";
    document.getElementById("gallary-words-word").style.width = "80%";
    document.getElementById("gallary-words-word").style.fontSize = "1.8em";
    document.getElementById("gallary-container").style.height = "500px";
    document.getElementById("gallary-container").style.display = "grid";
    document.getElementById("gallary-container").style.gridTemplateRows =
      "1fr 1fr 1fr 1fr 1fr";
    document.getElementById("gallary-container").style.gridTemplateColumns =
      "1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr";
    if (this.props.gallary.images.length > 10) {
      document.getElementById("gallary-0").style.gridRow = "1/4";
      document.getElementById("gallary-0").style.gridColumn = "1/4";
      document.getElementById("gallary-0").style.padding = "4px";

      document.getElementById("gallary-1").style.gridRow = "4/6";
      document.getElementById("gallary-1").style.gridColumn = "1/3";
      document.getElementById("gallary-1").style.padding = "4px";

      document.getElementById("gallary-2").style.gridRow = "4/6";
      document.getElementById("gallary-2").style.gridColumn = "3/6";
      document.getElementById("gallary-2").style.padding = "4px";

      document.getElementById("gallary-3").style.gridRow = "1/4";
      document.getElementById("gallary-3").style.gridColumn = "4/6";
      document.getElementById("gallary-3").style.padding = "4px";

      document.getElementById("gallary-4").style.gridRow = "1/4";
      document.getElementById("gallary-4").style.gridColumn = "6/8";
      document.getElementById("gallary-4").style.padding = "4px";

      document.getElementById("gallary-5").style.gridRow = "4/6";
      document.getElementById("gallary-5").style.gridColumn = "6/8";
      document.getElementById("gallary-5").style.padding = "4px";

      document.getElementById("gallary-6").style.gridRow = "1/2";
      document.getElementById("gallary-6").style.gridColumn = "8/9";
      document.getElementById("gallary-6").style.padding = "4px";

      document.getElementById("gallary-7").style.gridRow = "1/2";
      document.getElementById("gallary-7").style.gridColumn = "9/10";
      document.getElementById("gallary-7").style.padding = "4px";

      document.getElementById("gallary-8").style.gridRow = "1/2";
      document.getElementById("gallary-8").style.gridColumn = "10/11";
      document.getElementById("gallary-8").style.padding = "4px";

      document.getElementById("gallary-9").style.gridRow = "2/6";
      document.getElementById("gallary-9").style.gridColumn = "8/11";
      document.getElementById("gallary-9").style.padding = "4px";
    }
  }

  render() {
    let list = [];
    for (let i = 0; i < 10; i++) {
      list.push(this.props.gallary.images[i]);
    }
    return (
      <div id="gallary" className="gallary">
        <div id="gallary-words">
          <div id="gallary-words-word">Gallary</div>
        </div>
        <div id="gallary-container">
          {list.map((url, index) => {
            return (
              <div
                key={"gallary" + index}
                className="gallary-album"
                id={"gallary-" + index}
              >
                <img src={url} alt=""></img>
              </div>
            );
          })}
        </div>
      </div>
    );
  }
}
