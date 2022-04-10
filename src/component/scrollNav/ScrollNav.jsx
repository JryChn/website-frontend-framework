import { Component } from "react";
import style from "./ScrollNav.module.scss";

export default class ScrollNav extends Component {
  componentDidMount() {
    this.srollListener = window.addEventListener(
      "scroll",
      this.handleNavActive
    );
    this.navActiveListener = window.addEventListener(
      "click",
      this.handleNavActive
    );
  }
  componentWillUnmount() {
    window.removeEventListener(this.srollListener);
    window.removeEventListener(this.navActiveListener);
  }

  handleNavActive = () => {
    if (document.getElementById("introduce") === null) {
      return;
    }
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
      let id = sec.name + index;
      let top = document.getElementById(id).getBoundingClientRect().top;
      let bottom = document.getElementById(id).getBoundingClientRect().bottom;
      if (top < 0 && bottom > 0) {
        document.getElementById(sec.name + index + "id").style.borderBottom =
          "2px black solid";
        document.getElementById(
          sec.name + index + "id"
        ).style.transitionDuration = "0.5s";
      } else {
        document.getElementById(sec.name + index + "id").style.borderBottom =
          "none";
        document.getElementById(
          sec.name + index + "id"
        ).style.transitionDuration = "0.5s";
      }
    });
  };

  handleOnClick = (e) => {
    let target = e.target;
    let id = target.id.replace("id", "");
    let position = document.getElementById(id).offsetTop;
    window.scrollTo({
      top: position + 200,
      behavior: "smooth",
    });
  };

  render() {
    return (
      <div className={style.container} id="scrollnav">
        <div>
          <ul>
            {this.props.nav.map((navs, index) => {
              return (
                <li
                  id={navs.name + index + "id"}
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
