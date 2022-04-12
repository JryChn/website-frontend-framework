import { Component, useState } from "react";
import style from "./Header.module.scss";

export default class Header extends Component {
  constructor(props) {
    super(props);
    this.state = {
      isHumbeger: false,
      moreOptions: [
        {
          name: "Telegram",
          address: this.props.telegram,
        },
        {
          name: "Email",
          address: this.props.email,
        },
        {
          name: "Resume",
          address: "",
        },
        {
          name: "Calendar",
          address: "",
        },
      ],
    };
    window.onresize = () => {
      this.setState({
        isHumbeger: window.innerWidth <= 900 ? true : false,
      });
    };
  }
  switchOutlook = (isHumbeger) => {
    isHumbeger = window.innerWidth <= 900 ? true : false;
    if (isHumbeger) {
      return (
        <Humbeger
          humbeger={this.props.header.map((section) => {
            return section.name;
          })}
          moreOptions={this.state.moreOptions}
        ></Humbeger>
      );
    } else {
      return (
        <NavBar
          navItems={this.props.header.map((section) => {
            return section.name;
          })}
          moreOptions={this.state.moreOptions}
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
            {props.moreOptions.map((item, index) => {
              return (
                <li
                  key={"more-items" + item.name + index}
                  onClick={(e) => {
                    if (
                      /^([0-9A-Za-z-_.]+)@([0-9a-z]+.[a-z]{2,3}(.[a-z]{2})?)$/g.test(
                        item.address
                      )
                    ) {
                      window.location.href = "mailto:" + item.address;
                      return;
                    }
                    window.location.href = item.address;
                  }}
                >
                  {item.name}
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
                key={"header-humbeger" + n.name + index}
                onClick={(e) => jumpOnClick(e)}
              >
                {n}
              </li>
            );
          })}
          {props.moreOptions.map((n, index) => {
            return (
              <li
                key={"header-humbeger-more" + n.name + index}
                onClick={(e) => {
                  if (
                    /^([0-9A-Za-z-_.]+)@([0-9a-z]+.[a-z]{2,3}(.[a-z]{2})?)$/g.test(
                      n.address
                    )
                  ) {
                    window.location = "mailto:" + n.address;
                  }
                  window.location.href = n.address;
                }}
              >
                {n.name}
              </li>
            );
          })}
        </ul>
      </div>
    </div>
  );
}

function jumpOnClick(e, name, index) {
  let id = name + index;
  if (document.getElementById(id) === null) return;
  let position = document.getElementById(id).offsetTop;
  window.scrollTo({
    top: position + 200,
    behavior: "smooth",
  });
}
