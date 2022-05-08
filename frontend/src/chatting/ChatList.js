import { React, useState } from "react";
import { v4 as uuidv4 } from 'uuid';
import classNames from "classnames";

import Chat from "./Chat"

const ChatList = ({ message, currentChat, selectChat }) => {
	const [text, updateTextbox] = useState('')
	const [isSearch, toggleSearch] = useState(true)
	const [chats, updateChats] = useState([])

	currentChat = chats.length === 0
		? null
		: chats[0]

	const newChat = () => {
		if (text.length > 0 && text.length < 500) {
			let id = uuidv4()

			const newChats = chats
			newChats.push(<Chat
				id={id}
				key={id}
				address={text}
				message={message}
				isSelected={false}
				selectChat={selectChat}
			/>)

			updateChats(newChats)
			updateTextbox('')
		} else if (text.length === 0) {
			console.log('address empty')
		} else {
			console.log('address too long')
		}
	}

	return (
		<div className="flex p-4 bg-gray-100 bg-opacity-25 border-r border-gray-200 w-1/5 h-screen">
			<div className="flex-col w-96 my-2 pb-6 px-2">
				<input className="block px-3 w-full py-2 border border-gray-300
					bg-gray-200 placeholder-gray-500 text-gray-900 rounded focus:outline-none
					focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
					placeholder={isSearch ? 'Search' : 'Enter address'}
					value={text}
					onChange={e => updateTextbox(e.target.value)}
					onKeyUp={e => !isSearch && e.key === 'Enter' ? newChat() : null}
				/>
				<div>
					{
						chats.length === 0
							? <h1 className="my-8 text-gray-500 text-center">
								It's so empty here...<br />
								Maybe add a new chat?</h1>
							: chats
					}
				</div>
			</div>
			<img src={require('../assets/new_msg.png')}
				className={classNames({
					"mx-4 my-1 w-10 h-10 cursor-pointer transition ease-in-out duration-500 hover:scale-110": true,
					"hidden": !isSearch
				})}
				onClick={() => toggleSearch(false)}
			/>
			<img src={require('../assets/plus.png')}
				className={classNames({
					"mx-4 my-1 w-10 h-10 cursor-pointer transition ease-in-out duration-500 hover:scale-110": true,
					"hidden": isSearch
				})}
				onClick={newChat}
			/>
			<img src={require('../assets/back.png')}
				className={classNames({
					"mx-4 my-1 w-10 h-10 cursor-pointer transition ease-in-out duration-500 hover:scale-110": true,
					"hidden": isSearch
				})}
				onClick={() => toggleSearch(true)}
			/>
		</div>
	)
}

export default ChatList;