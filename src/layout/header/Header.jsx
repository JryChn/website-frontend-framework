import { Component, useState } from "react";
import style from "./Header.module.scss";

const moreOptions = ["Telegram", "Email", "Resume", "MyCalendar"];

export default class Header extends Component {
  constructor(props) {
    super(props);
    this.state = { isHumbeger: false };
    window.onresize = () => {
      this.setState({
        isHumbeger: window.innerWidth <= 900 ? true : false,
      });
    };
  }
  switchOutlook = (isHumbeger) => {
    if (isHumbeger) {
      return (
        <Humbeger
          humbeger={this.props.header.map((section) => {
            return section.name;
          })}
        ></Humbeger>
      );
    } else {
      return (
        <NavBar
          navItems={this.props.header.map((section) => {
            return section.name;
          })}
        ></NavBar>
      );
    }
  };

  render() {
    return (
      <div>
        <header className={style.header}>
          <div className={style.logo}>
            <a href={this.props.web_url}>
              <img src={this.props.logo} alt={this.props.title} />
            </a>
          </div>
          {this.switchOutlook(this.state.isHumbeger)}
        </header>
      </div>
    );
  }
}

function NavBar(props) {
  return (
    <div className={style.nav}>
      <ul>
        {props.navItems.map((item, index) => {
          return (
            <li
              key={"header" + item + index}
              onClick={(e) => jumpOnClick(e, item, index)}
            >
              {item}
            </li>
          );
        })}
      </ul>
      <div className={style.more}>
        <span>More</span>
        <div className={style.items}>
          <ul>
            {moreOptions.map((item, index) => {
              return (
                <li
                  key={"more-items" + item + index}
                  onClick={(e) => jumpOnClick(e, index)}
                >
                  {item}
                </li>
              );
            })}
          </ul>
        </div>
      </div>
    </div>
  );
}

function Humbeger(props) {
  const [visible, setVisible] = useState(true);
  let handleOnClick = (e) => {
    if (visible) {
      e.target.nextSibling.style.display = "block";
    } else {
      e.target.nextSibling.style.display = "none";
    }
    setVisible(!visible);
  };
  return (
    <div className={style.humbeger}>
      <div className={style.humbegerBox} onClick={handleOnClick}>
        <div></div>
        <div></div>
        <div></div>
      </div>
      <div>
        <ul>
          {props.humbeger.map((n, index) => {
            return (
              <li
                key={"header-humbeger" + n + index}
                onClick={(e) => jumpOnClick(e)}
              >
                {n}
              </li>
            );
          })}
          {moreOptions.map((n, index) => {
            return <li key={"header-humbeger-more" + n + index}>{n}</li>;
          })}
        </ul>
      </div>
    </div>
  );
}

function jumpOnClick(e, name, index) {
  let id = name + index;
  let position = document.getElementById(id).offsetTop;
  window.scrollTo({
    top: position + 200,
    behavior: "smooth",
  });
}
