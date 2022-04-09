import { Component } from "react";
import Header from "./layout/header/Header";
import MainContent from "./layout/MainContent";
import Footer from "./layout/footer/Footer";
import BlogPage from "./layout/blogPage/BlogPage";
import { Routes, Route } from "react-router-dom";
import config from "./config.json";

export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      config: config,
    };
    document.title = config.title;
  }

  render() {
    return (
      <div>
        <Header
          web_url={this.state.config.url}
          logo={this.state.config.logo}
          header={this.state.config.section}
          title={this.state.config.title}
        ></Header>
        <Routes>
          <Route
            path="/"
            element={
              <MainContent
                introduce={this.state.config.introduce}
                main={this.state.config.section}
              />
            }
          />
          <Route
            path="blog"
            element={
              <BlogPage
                blogs={() => {
                  let address;
                  this.state.config.section.map((item) => {
                    if (item.model === 1) {
                      address = item.address;
                    }
                    return address;
                  });
                }}
              />
            }
          />
          <Route path="story" element={<App />} />
        </Routes>
        <Footer footer={this.state.config.footer}></Footer>
      </div>
    );
  }
}
