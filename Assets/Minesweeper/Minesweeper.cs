using System;
using Kdletters.EventSystem;
using RustLib;
using static RustLib.Lib;
using UnityEngine;
using UnityEngine.UI;

namespace Minesweeper
{
    public unsafe class Minesweeper : MonoBehaviour
    {
        public class Config
        {
            public const string Flag = "<color=red>旗</color>";
            public const string Mine = "雷";
        }

        [SerializeField] private Block _prefab;
        [SerializeField] private GridLayoutGroup _grid;
        [SerializeField] private int _width = 10;
        [SerializeField] private int _height = 10;
        [SerializeField] private int _mineCount = 10;

        private static MinesweeperGame* _game;

        private static void OnGameOver(bool isWin, int x, int y)
        {
            Console.Instance.Log($"GameOver: {isWin} ({x}, {y})");
        }

        private static void OnOpenBlock(int x, int y)
        {
            Console.Instance.Log($"OnOpenBlock: ({x}, {y})");
            KEventSystem.Dispatch(new RefreshBlock(get_block(_game, x, y)));
        }

        private void Awake()
        {
            _game = create_game(_width, _height, _mineCount);
            set_on_game_over(_game, OnGameOver);
            set_on_open_block(_game, OnOpenBlock);

            KEventSystem.Subscribe<ClickBlock>(OnClickBlock);
            KEventSystem.Subscribe<FlagBlock>(OnFlagBlock);
        }

        private void Start()
        {
            for (int i = 0; i < _height; i++)
            {
                for (int j = 0; j < _width; j++)
                {
                    Instantiate(_prefab, _grid.transform).Set(j, i);
                }
            }
        }

        private void OnDestroy()
        {
            free_game(_game);
            _game = null;

            KEventSystem.Unsubscribe<ClickBlock>(OnClickBlock);
            KEventSystem.Unsubscribe<FlagBlock>(OnFlagBlock);
        }

        private void OnClickBlock(in ClickBlock arg)
        {
            if (open_block(_game, arg.X, arg.Y))
            {
                KEventSystem.Dispatch(new RefreshBlock(get_block(_game, arg.X, arg.Y)));
            }
        }

        private void OnFlagBlock(in FlagBlock arg)
        {
            if (flag_block(_game, arg.X, arg.Y))
            {
                KEventSystem.Dispatch(new RefreshBlock(get_block(_game, arg.X, arg.Y)));
            }
        }
    }
}

namespace System.Runtime.CompilerServices
{
    public class IsExternalInit : Attribute
    {
    }
}

public record ClickBlock(int X, int Y);
public record FlagBlock(int X, int Y);

public unsafe struct RefreshBlock
{
    public Block* Block;

    public RefreshBlock(Block* block)
    {
        Block = block;
    }
}