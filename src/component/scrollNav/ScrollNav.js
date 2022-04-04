import { Component } from "react";
import "./ScrollNav.css";

export default class ScrollNav extends Component {
  componentDidMount() {
    window.addEventListener("scroll", this.handleNavActive);
    window.addEventListener("click", this.handleNavActive);
  }

  handleNavActive = () => {
    let rect = document
      .getElementById("introduce")
      .getBoundingClientRect().bottom;
    if (rect < 0) {
      document.getElementById("scrollnav").style.display = "block";
      if (rect < -100) {
        document.getElementById("scrollnav").style.opacity = 1;
      } else {
        document.getElementById("scrollnav").style.opacity = 0;
      }
    } else {
      document.getElementById("scrollnav").style.display = "none";
    }
    this.handleItemsActive();
  };

  handleItemsActive = () => {
    this.props.nav.map((sec, index) => {
      let id;
      switch (sec.model) {
        case 1:
          id = "blog";
          break;
        case 2:
          id = "aboutme";
          break;
        case 3:
          id = "gallary";
          break;
        default:
          id = "story";
      }
      let top = document.getElementById(id).getBoundingClientRect().top;
      let bottom = document.getElementById(id).getBoundingClientRect().bottom;
      if (top < 0 && bottom > 0) {
        document.getElementById("navs" + index + "id").style.borderBottom =
          "2px black solid";
        document.getElementById(
          "navs" + index + "id"
        ).style.transitionDuration = "0.5s";
      } else {
        document.getElementById("navs" + index + "id").style.borderBottom =
          "none";
        document.getElementById(
          "navs" + index + "id"
        ).style.transitionDuration = "0.5s";
      }
    });
  };

  handleOnClick = (e) => {
    let target = e.target;
    let id = target.id.replace("navs", "").replace("id", "");
    switch (id) {
      case "0":
        id = "blog";
        break;
      case "1":
        id = "aboutme";
        break;
      case "2":
        id = "gallary";
        break;
      default:
        id = "story";
    }
    let position = document.getElementById(id).offsetTop;
    window.scrollTo({
      top: position + 200,
      behavior: "smooth",
    });
  };

  render() {
    return (
      <div id="scrollnav">
        <div id="scrollnav-container">
          <ul>
            {this.props.nav.map((navs, index) => {
              return (
                <li
                  id={"navs" + index + "id"}
                  key={"navs" + index}
                  onClick={(e) => this.handleOnClick(e)}
                >
                  {navs.name}
                </li>
              );
            })}
          </ul>
        </div>
      </div>
    );
  }
}
