import React, { createContext, useState } from "react";
import { useAuth0, withAuthenticationRequired } from "@auth0/auth0-react";

import ChatNav from './chatting/ChatNav';
import Conversation from "./chatting/Conversation";
import Profile from "./auth/Profile";


function App() {
	const [message, newMessage] = useState('')
	const [selectedChat, selectChat] = useState(null)

	return (
		<div className="flex">
			{/* <ChatList message={message} selectedChat={selectedChat} selectChat={selectChat}/> */}
			<div className="bg-gray-800 w-1/4">
				<Profile />
				<ChatNav />
			</div>
			<Conversation newMessage={newMessage} />
		</div>
	)
}

export const MessageContext = createContext(null)
export default withAuthenticationRequired(App, { returnTo: "/"});