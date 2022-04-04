import React from "react";
import "./Story.css";

export default class Story extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      storys: [
        { img: "https://source.unsplash.com/random/?city,night" },
        { img: "https://source.unsplash.com/random/?fruit" },
        { img: "https://source.unsplash.com/random/?city,night" },
        { img: "https://source.unsplash.com/random/?city,night" },
        { img: "https://source.unsplash.com/random/?city,night" },
        { img: "https://source.unsplash.com/random/?city,night" },
      ],
    };
  }
  render() {
    return (
      <div id="story" className="story">
        <div id="story-words">
          <div>
            <h6>{this.props.story.name}</h6>
            <h2>{this.props.story.description}</h2>
          </div>
        </div>
        <div id="story-container">
          <div className="story-button" id="story-button-left"></div>
          <div className="story-button" id="story-button-right"></div>
          <div id="story-selector">
            {this.state.storys.map((story, index) => {
              return (
                <div className="story-content" key={"story" + index}>
                  <img src={story.img} alt="" />
                </div>
              );
            })}
          </div>
        </div>
      </div>
    );
  }
}
