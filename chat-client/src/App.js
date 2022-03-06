import React, { createContext, useState } from "react";
import ChatList from "./ChatList";
import Conversation from "./Conversation";

import './styles/output.css'


function App() {
	const [message, newMessage] = useState('')
	const [selectedChat, selectChat] = useState(null)

	return (
		<div className="flex">
			<ChatList message={message} selectedChat={selectedChat} selectChat={selectChat}/>
			<Conversation newMessage={newMessage} />
		</div>
	)
}

export const MessageContext = createContext(null)
export default App;