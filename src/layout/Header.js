import { Component } from "react";
import "./Header.css";
import NavBar from "../component/navBar/NavBar";
import { handleUrl } from "../utils/utils";

const web_title = document.title;
export default class Header extends Component {
  render() {
    return (
      <div id="header">
        <a href={this.props.web_url}>
          <img
            id="header-logo"
            src={handleUrl(this.props.header.logo)}
            alt={web_title}
          />
        </a>
        <NavBar
          nav={this.props.header.nav.map((head) => {
            return head.name;
          })}
        ></NavBar>
      </div>
    );
  }
}
