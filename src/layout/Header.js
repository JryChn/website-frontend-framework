import { Component } from "react";
import "./Header.css";
import NavBar from "../component/navBar/NavBar";

export default class Header extends Component {
  render() {
    return (
      <div id="header">
        <a href={this.props.web_url}>
          <img id="header-logo" src={this.props.logo} alt={this.props.title} />
        </a>
        <NavBar
          nav={this.props.header.map((section) => {
            return section.name;
          })}
        ></NavBar>
      </div>
    );
  }
}
