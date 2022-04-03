import React from "react";
import "./Story.css";

export default class Story extends React.Component {
  render() {
    return (
      <div id="story" className="story">
        <div id="story-words">
          <div>words</div>
        </div>
        <div id="story-container">
          <div id="story-selector"></div>
        </div>
      </div>
    );
  }
}
