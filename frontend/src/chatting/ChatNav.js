import React from 'react'
import { Disclosure } from '@headlessui/react'
import ChatList from './ChatList'

export default function ChatNav() {
    return (
        <Disclosure>
            <Disclosure.Panel static>
                <div className="flex-col my-2 pb-6 px-4">
                    <p className='text-gray-300 text-xl m-4'>Chats</p>
                    <input
                        className="focus:border-indigo-500 block w-full pl-7 pr-12 py-2 border-gray-300 rounded-md"
                        placeholder="Search"
                    />
                    <ChatList />
                </div>
            </Disclosure.Panel>
        </Disclosure>
    )
}