import React, { createContext, useState } from "react";
import { useAuth0, withAuthenticationRequired } from "@auth0/auth0-react";

import ChatList from "./chatting/ChatList";
import Conversation from "./chatting/Conversation";
import Navbar from "./auth/Navbar";


function App() {
	const [message, newMessage] = useState('')
	const [selectedChat, selectChat] = useState(null)

	return (
		<div className="flex">
			<ChatList message={message} selectedChat={selectedChat} selectChat={selectChat}/>
			<div className="w-full h-screen">
				<Navbar />
				<Conversation newMessage={newMessage} />
			</div>
		</div>
	)
}

export const MessageContext = createContext(null)
export default withAuthenticationRequired(App, { returnTo: "/"});