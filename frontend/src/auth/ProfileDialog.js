/* This example requires Tailwind CSS v2.0+ */
import { Fragment } from 'react'
import { Dialog, Transition } from '@headlessui/react'
import { useAuth0 } from '@auth0/auth0-react'

export default function ProfileDialog(props) {
    const { user, isLoading } = useAuth0();

    if (isLoading)
        return <div>Loading ...</div>;

    return (
        <Transition.Root show={props.open} as={Fragment}>
            <Dialog as="div" className="relative z-10" onClose={() => props.setOpen(false)}>
                <Transition.Child
                    as={Fragment}
                    enter="ease-out duration-300"
                    enterFrom="opacity-0"
                    enterTo="opacity-100"
                    leave="ease-in duration-200"
                    leaveFrom="opacity-100"
                    leaveTo="opacity-0"
                >
                    <div className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
                </Transition.Child>

                <div className="fixed z-10 inset-0 overflow-y-auto">
                    <div className="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
                        {/* This element is to trick the browser into centering the modal contents. */}
                        <span className="hidden sm:inline-block sm:align-middle sm:h-screen" aria-hidden="true">
                            &#8203;
                        </span>
                        <Transition.Child
                            as={Fragment}
                            enter="ease-out duration-300"
                            enterFrom="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
                            enterTo="opacity-100 translate-y-0 sm:scale-100"
                            leave="ease-in duration-200"
                            leaveFrom="opacity-100 translate-y-0 sm:scale-100"
                            leaveTo="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
                        >
                            <Dialog.Panel className="relative inline-block bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all my-8 align-middle w-2/5">
                                <div className="shadow p-10 bg-white overflow-hidden">
                                    <h3 className="text-xl font-medium my-4 leading-6 text-gray-900">Your profile</h3>

                                    <div className="flex flex-col">
                                        <p className='text-gray-700 text-lg'>Photo</p>
                                        <div className="mt-1 flex items-center">
                                            <img
                                                src={user.picture}
                                                className="h-16 w-16 rounded-full"
                                            />
                                            <button
                                                type="button"
                                                className="ml-5 bg-white py-2 px-3 border border-gray-300 rounded-md shadow-sm text-sm leading-4 font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                                            >
                                                Change
                                            </button>
                                        </div>
                                    </div>

                                    <div className="flex flex-col">
                                        <div className='my-4'>
                                            <p className='text-gray-700 text-lg'>Username</p>
                                            <input
                                                className="border focus:border-indigo-500/75 block w-11/12 pl-7 pr-12 py-2 border-gray-300 rounded-md shadow-sm"
                                                placeholder={user.nickname == null ? 'First name' : user.nickname}
                                            />
                                        </div>

                                        <div className='my-4'>
                                            <p className='text-gray-700 text-lg'>Email address</p>
                                            <input
                                                className="border focus:border-indigo-500/75 block w-11/12 pl-7 pr-12 py-2 border-gray-300 rounded-md shadow-sm"
                                                placeholder={user.email == null ? 'Email' : user.email}
                                            />
                                        </div>
                                    </div>
                                </div>

                                <div className="bg-gray-50 px-6 py-3 flex flex-row-reverse">
                                    <button
                                        type="button"
                                        className="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-red-600 text-base font-medium text-white hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 sm:ml-3 sm:w-auto sm:text-sm"
                                        onClick={() => props.setOpen(false)}
                                    >
                                        Delete your account
                                    </button>
                                    <button
                                        type="button"
                                        className="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-700 hover:bg-indigo-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                                        onClick={() => props.setOpen(false)}
                                    >
                                        Save
                                    </button>
                                </div>
                            </Dialog.Panel>
                        </Transition.Child>
                    </div>
                </div>
            </Dialog>
        </Transition.Root>
    )
}