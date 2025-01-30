import React from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import "./index.css";
import ErrorPage from "./pages/ErrorPage.tsx";
//import { HomePage } from "./pages/HomePage.tsx";
import { GamePage } from "./pages/GamePage.tsx";
import { AppContextProvider } from "./GloabalState/context/AppContext.tsx";

const router = createBrowserRouter([
	{
		path: "/",
		element: <GamePage />,
		errorElement: <ErrorPage />,
	},
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<React.StrictMode>
		<AppContextProvider>
			<RouterProvider router={router} />
		</AppContextProvider>
	</React.StrictMode>
);
