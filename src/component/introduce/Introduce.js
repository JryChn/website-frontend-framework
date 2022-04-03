import { Component } from "react";
import "./Introduce.css";
import { handleUrl } from "../../utils/utils";

export default class Introduce extends Component {
  render() {
    return (
      <div id="introduce">
        <div id="introduce-word1">
          <h4>{this.props.introduce.subTitle}</h4>
          <h1>{document.title + ".com"}</h1>
          <h6>{this.props.introduce.words}</h6>
        </div>
        <div id="introduce-image1">
          <img src={handleUrl(this.props.introduce.mainImage)} alt=""></img>
        </div>
        <div id="introduce-image2">
          <img src={handleUrl(this.props.introduce.subImage)} alt=""></img>
        </div>
        <div id="introduce-word2">
          <h6 id="introduce-word2-content">
            {'"' + this.props.introduce.quote.content + '"'}
          </h6>
          <h6 id="introduce-word2-celebrity">
            {"- " + this.props.introduce.quote.celebrity}
          </h6>
        </div>
      </div>
    );
  }
}
