import React from "react";
import { useLayoutEffect } from "react";
import ReactDOM from "react-dom";
import App from "./App";
import { BrowserRouter, useLocation } from "react-router-dom";
import "./style/index.scss";

const Wrapper = ({ children }) => {
  const location = useLocation();
  useLayoutEffect(() => {
    document.documentElement.scrollTo(0, 0);
  }, [location.pathname]);
  return children;
};

ReactDOM.render(
  <React.StrictMode>
    <BrowserRouter>
      <Wrapper>
        <App />
      </Wrapper>
    </BrowserRouter>
  </React.StrictMode>,
  document.getElementById("root")
);
