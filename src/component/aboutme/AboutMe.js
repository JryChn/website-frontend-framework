import React from "react";
import "./AboutMe.css";

export default class AbouMe extends React.Component {
  render() {
    return (
      <div className="aboutme" id="aboutme">
        <div id="aboutme-words">
          <div>
            <h6>{this.props.aboutme.name}</h6>
            <h2>{this.props.aboutme.description}</h2>
          </div>
        </div>
        <div id="aboutme-protaits">
          <section id="protaits-1">
            <div id="protait-icon-1"></div>
            <div id="protait-words-1">
              <span>{this.props.aboutme.trait1}</span>
            </div>
          </section>
          <section id="protaits-2">
            <div id="protait-icon-2"></div>
            <div id="protait-words-2">
              <span>{this.props.aboutme.trait2}</span>
            </div>
          </section>
          <section id="protaits-3">
            <div id="protait-icon-3"></div>
            <div id="protait-words-3">
              <span>{this.props.aboutme.trait3}</span>
            </div>
          </section>
        </div>
        <div id="aboutme-card-container">
          <div id="aboutme-card">
            <img src={this.props.aboutme.background} alt="" />
            <div id="aboutme-name">{this.props.aboutme.who}</div>
            <div id="aboutme-description">{this.props.aboutme.oneWords}</div>
            <div id="aboutme-protait">
              <img src={this.props.protait} alt=""></img>
            </div>
            <div id="aboutme-career">{this.props.aboutme.position}</div>
          </div>
        </div>
      </div>
    );
  }
}
