import React, { createContext, useState } from "react";
import { useAuth0, withAuthenticationRequired } from "@auth0/auth0-react";

import ChatNav from './chatting/ChatNav';
import Conversation from "./chatting/Conversation";
import ProfileSmall from "./auth/ProfileSmall";


function App() {
	const [message, newMessage] = useState('')
	const [selectedChat, selectChat] = useState(null)

	return (
		<div className="flex">
			{/* <ChatList message={message} selectedChat={selectedChat} selectChat={selectChat}/> */}
			<div className="bg-gray-100 bg-opacity-25 border-r w-1/4">
				<ProfileSmall />
				<ChatNav />
			</div>
			<Conversation newMessage={newMessage} />
		</div>
	)
}

export const MessageContext = createContext(null)
export default withAuthenticationRequired(App, { returnTo: "/"});