import { Component } from "react";
import Introduce from "../component/introduce/Introduce";
import Blog from "../component/blog/Blog";
import AboutMe from "../component/aboutme/AboutMe";
import Gallary from "../component/gallary/Gallary";
import Story from "../component/story/Story";
import ScrollNav from "../component/scrollNav/ScrollNav";

export default class MainContent extends Component {
  render() {
    return (
      <div>
        <Introduce introduce={this.props.introduce}></Introduce>
        <ScrollNav nav={this.props.main}></ScrollNav>
        {this.props.main.map((section, index) => {
          switch (section.model) {
            case 1:
              return <Blog key={"main" + index} blog={section}></Blog>;
            case 2:
              return <AboutMe key={"main" + index} aboutme={section}></AboutMe>;
            case 3:
              return <Gallary key={"main" + index} gallary={section}></Gallary>;
            default:
              return <Story key={"main" + index} story={section}></Story>;
          }
        })}
      </div>
    );
  }
}
