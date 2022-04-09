import { Component } from "react";
import Introduce from "../component/introduce/Introduce";
import Outlook1 from "../component/outlook1/Outlook1";
import Outlook2 from "../component/outlook2/Outlook2";
import Outlook3 from "../component/outlook3/Outlook3";
import Outlook4 from "../component/outlook4/Outlook4";
import ScrollNav from "../component/scrollNav/ScrollNav";

export default class MainContent extends Component {
  render() {
    return (
      <div>
        <Introduce introduce={this.props.introduce}></Introduce>
        <ScrollNav nav={this.props.main}></ScrollNav>
        {this.props.main.map((module, index) => {
          switch (module.model) {
            case 1:
              return (
                <Outlook1 key={"main" + index} content={module} index={index}>
                  {" "}
                </Outlook1>
              );
            case 2:
              return (
                <Outlook2
                  key={"main" + index}
                  content={module}
                  index={index}
                ></Outlook2>
              );
            case 3:
              return (
                <Outlook3
                  key={"main" + index}
                  content={module}
                  index={index}
                ></Outlook3>
              );
            default:
              return (
                <Outlook4
                  key={"main" + index}
                  content={module}
                  index={index}
                ></Outlook4>
              );
          }
        })}
      </div>
    );
  }
}
