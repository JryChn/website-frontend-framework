import { Component } from "react";
import Header from "./layout/header/Header";
import MainContent from "./layout/MainContent";
import Footer from "./layout/footer/Footer";
import Page from "./layout/blogPage/Page";
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
            index
            path="/"
            element={
              <MainContent
                introduce={this.state.config.introduce}
                main={this.state.config.section}
              />
            }
          />
          <Route path="blog">
            <Route
              index
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
            <Route path=":id" element={<Page />} />
          </Route>
          <Route path="story" element={<App />} />
          <Route
            path="*"
            element={
              <main style={{ padding: "1rem" }}>
                <p>There's nothing here!</p>
              </main>
            }
          />
        </Routes>
        <Footer footer={this.state.config.footer}></Footer>
      </div>
    );
  }
}
