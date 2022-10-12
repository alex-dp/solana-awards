// Next, React
import { FC, useEffect, useState } from 'react';
import Link from 'next/link';

// Wallet
import { useWallet, useConnection } from '@solana/wallet-adapter-react';

// Components
import { RequestAirdrop } from '../../components/RequestAirdrop';
import pkg from '../../../package.json';

// Store
import useUserSOLBalanceStore from '../../stores/useUserSOLBalanceStore';

export const HomeView: FC = ({ }) => {
  const wallet = useWallet();
  const { connection } = useConnection();

  const balance = useUserSOLBalanceStore((s) => s.balance)
  const { getUserSOLBalance } = useUserSOLBalanceStore()

  useEffect(() => {
    if (wallet.publicKey) {
      console.log(wallet.publicKey.toBase58())
      getUserSOLBalance(wallet.publicKey, connection)
    }
  }, [wallet.publicKey, connection, getUserSOLBalance])

  return (

    <div className="md:hero mx-auto p-4">
      <div className="md:hero-content flex flex-col">
        <h1 className="text-center text-5xl md:pl-12 font-bold text-transparent bg-clip-text bg-gradient-to-tr from-[#FFD700] to-[#B59410]">
          S0L Academy Awards<span className='text-sm font-normal align-top text-slate-700'>v{pkg.version}</span>
        </h1>
        <h4 className="md:w-full text-center text-slate-300 my-2">
          <p>Simply the fastest way to get started.</p>
          Next.js, tailwind, wallet, web3.js, and more.
        </h4>
        <div className="max-w-md mx-auto mockup-code bg-primary p-6 my-2">
          <pre data-prefix=">">
            <code className="truncate">Start building on Solana  </code>
          </pre>
        </div> 
        <div className="container grid grid-cols-3 gap-2 mx-auto">
  
          <div className="overflow-hidden  aspect-video bg-red-400 cursor-pointer rounded-xl relative group">
         
            <div
                className="rounded-xl z-50 opacity-0 group-hover:opacity-100 transition duration-300 ease-in-out cursor-pointer absolute from-black/80 to-transparent bg-gradient-to-t inset-x-0 -bottom-2 pt-30 text-white flex items-end"
            >
                <div>
                    <div
                        className="transform-gpu  p-4 space-y-3 text-xl group-hover:opacity-100 group-hover:translate-y-0 translate-y-4 pb-10 transform transition duration-300 ease-in-out"
                    >
                        <div className="font-bold">Tub Apes</div>

                        <div className="opacity-60 text-sm ">
                            Lorem ipsum dolor sit amet, consectetur adipisicing
                            elit. Distinctio dolores error iure, perferendis
                            sequi totam. 
                        </div>
                    </div>
                </div>
                
            </div><img src="https://i.seadn.io/gae/IUcp9-GTzGOjatQ7tgXHTpZEjYowyKHFRneqLZwkHBwnWUKaCmfTytOjdRejHkmQhQClSXvRwZJ5zhXWiLGu0x_stCK8v1pQTRXE?auto=format&w=384"
              alt="image"/>
            </div>

          <div className="overflow-hidden  aspect-video bg-red-400 cursor-pointer rounded-xl relative group">
              <img src="https://i.seadn.io/gae/IUcp9-GTzGOjatQ7tgXHTpZEjYowyKHFRneqLZwkHBwnWUKaCmfTytOjdRejHkmQhQClSXvRwZJ5zhXWiLGu0x_stCK8v1pQTRXE?auto=format&w=384"
              alt="image"/>
          </div>
          <div className="overflow-hidden  aspect-video bg-red-400 cursor-pointer rounded-xl relative group">
              <img src="https://i.seadn.io/gae/UxDofIDkdYcfMXml3rS9iOnaCcum-BkDH76xqZHkWaSvJsuCppEa7-slAYb8qqI1hc2MxIlxSvMbc97l21cdBvWTY2oGcELXf-llHA?auto=format&w=384"
              alt="image"/>
          </div>
          <div className="overflow-hidden  aspect-video bg-red-400 cursor-pointer rounded-xl relative group">
              <img src="https://i.seadn.io/gae/BQybtr_qVWIMO034lYWy8VU45O8lqUf7eaQd9-BzBQ3_ixXk8uJFAMX8NUAYkn0B4LQ_ukyjpkztmV57aWbG8rif4Olo9C6-o8lkQA?auto=format&w=384"
              alt="image"/>
          </div>
          <div className="overflow-hidden  aspect-video bg-red-400 cursor-pointer rounded-xl relative group">
              <img src="https://i.seadn.io/gae/svZX_BTwS9ks24jwM7oX4E0kDw-LiB8mzkq8PCCEFQAHA_Fy5qiU_WaBlv1ILm_CFjfJqTW78Tug3YN8F8-hK76avbJ20qwGDHcI?auto=format&w=384"
              alt="image"/>
          </div>
          <div className="overflow-hidden  aspect-video bg-red-400 cursor-pointer rounded-xl relative group">
              <img src="https://i.seadn.io/gae/m8JtOCMUzUkTP0Wp_T9P194PH2NsEwe6Z8_lgKfWbhJs1kmp_Wr2h9Hq5km0EXq4RYMIlgl-ezli9hofsUTqFigE4UIYA37uIDxesQ?auto=format&w=384"
              alt="image"/>
          </div>
          <div className="overflow-hidden  aspect-video bg-red-400 cursor-pointer rounded-xl relative group">
              <img src="https://i.seadn.io/gae/D_4TWEi3vTSHbZnU8fOXxLlKynTM9CKaNIGaiiUwbln9insoLZDM8NiB5_PrX5xNIfZIAG1WNbOdVh0q18ga2xOlQI43EYcpTDA4lA?auto=format&w=384"
              alt="image"/>
          </div>
          <div className="overflow-hidden  aspect-video bg-red-400 cursor-pointer rounded-xl relative group">
              <img src="https://i.seadn.io/gae/m8JtOCMUzUkTP0Wp_T9P194PH2NsEwe6Z8_lgKfWbhJs1kmp_Wr2h9Hq5km0EXq4RYMIlgl-ezli9hofsUTqFigE4UIYA37uIDxesQ?auto=format&w=384"
              alt="image"/>
          </div>
          <div className="overflow-hidden  aspect-video bg-red-400 cursor-pointer rounded-xl relative group">
              <img src="https://i.seadn.io/gae/m8JtOCMUzUkTP0Wp_T9P194PH2NsEwe6Z8_lgKfWbhJs1kmp_Wr2h9Hq5km0EXq4RYMIlgl-ezli9hofsUTqFigE4UIYA37uIDxesQ?auto=format&w=384"
              alt="image"/>
          </div>
      
          

        </div>
               
          <div className="text-center">
          <RequestAirdrop />
          {/* {wallet.publicKey && <p>Public Key: {wallet.publicKey.toBase58()}</p>} */}
          {wallet && <p>SOL Balance: {(balance || 0).toLocaleString()}</p>}
        </div>
      </div>
    </div>
  );
};
