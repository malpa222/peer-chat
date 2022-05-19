import { Fragment, useState } from 'react'
import { Menu, Transition } from '@headlessui/react'
import { useAuth0 } from '@auth0/auth0-react'

import ProfileDialog from './ProfileDialog'

function classNames(...classes) {
	return classes.filter(Boolean).join(' ')
}

export default function ProfileSmall() {
	const { user, logout, isLoading } = useAuth0();
    const [open, setOpen] = useState(false)

	if (isLoading) {
		return <div>Loading ...</div>;
	}

	const getUser = async function() {
		let res = await fetch(`http://127.0.0.1:8081/users/get?id=0&auth0_id=${user.sub}`)
		if (res.status > 400) {
			logout()
		}
	}
	getUser()

	return (
		<div className='m-4 py-4'>
			<div className='flex items-center'>
				<Menu as="div" className="relative">
					<div className='flex items-center'>
						<Menu.Button className="bg-gray-800 mx-2 flex text-sm rounded-full shadow-2xl focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-white">
							<img
								src={user.picture}
								className="h-12 w-12 rounded-full"
							/>
						</Menu.Button>
						<p className='text-black-300 mx-2 w-36 text-md'>{user.nickname}</p>
					</div>
					<ProfileDialog open={open} setOpen={setOpen}/>
					<Transition
						as={Fragment}
						enter="transition ease-out duration-100"
						enterFrom="transform opacity-0 scale-95"
						enterTo="transform opacity-100 scale-100"
						leave="transition ease-in duration-75"
						leaveFrom="transform opacity-100 scale-100"
						leaveTo="transform opacity-0 scale-95"
					>
						<Menu.Items className="origin-top-right absolute right-0 mt-2 w-48 rounded-md shadow-lg py-1 bg-white ring-1 ring-black ring-opacity-5 focus:outline-none">
							<Menu.Item>
								{({ active }) => (
									<a
										href="#"
										className={classNames(active ? 'bg-gray-200' : '', 'block px-4 py-2 text-sm text-black-700')}
										onClick={() => setOpen(!open)}
									>
										Your profile
									</a>
								)}
							</Menu.Item>
							<Menu.Item>
								{({ active }) => (
									<a
										href="#"
										className={classNames(active ? 'bg-red-200' : '', 'block px-4 py-2 text-sm text-black-700')}
										onClick={() => logout({ returnTo: window.location.origin })}
									>
										Sign out
									</a>
								)}
							</Menu.Item>
						</Menu.Items>
					</Transition>
				</Menu>
			</div>
		</div>
	)
}